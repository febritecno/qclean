//! App leftovers: `~/Library` data whose app is no longer installed.
//!
//! This is deliberately conservative, because unlike a cache these files are
//! not regenerated. Two kinds of entry are reported:
//!
//! - **Named after a bundle identifier** (`com.vendor.App`, in any of
//!   [`LEFTOVER_LOCATIONS`]): reported when *no* installed app, launch agent or
//!   helper shares its vendor prefix (`com.vendor`). Leftovers of one removed
//!   app are therefore kept while a sibling app from the same vendor is still
//!   installed. Apple's own identifiers are never reported.
//! - **Named after the app itself** (`~/Library/Application Support/Some App`):
//!   reported only when no installed app has a similar name, no macOS
//!   framework or daemon has a similar name, *and* nothing inside has been
//!   modified for [`STALE_AFTER`]. The age test is what keeps data of software
//!   that lives outside the Applications folders (games, SDKs) off the list.

use crate::model::Category;
use std::collections::HashSet;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

pub const LABEL: &str = "Leftover from uninstalled app";
pub const STALE_LABEL: &str = "Possible leftover, unused for 90+ days";
pub const CATEGORY: Category = Category::AppLeftovers;

/// Our own bundle identifier (`identifier` in `tauri.conf.json`). A dev build
/// does not run from `/Applications`, so it would otherwise report itself.
const SELF_ID: &str = "com.febritecno.qclean";

/// How deep under an applications folder a `.app` bundle may sit
/// (`/Applications/Utilities/X.app`, `/Applications/Setapp/X.app`).
const APP_SEARCH_DEPTH: usize = 3;

/// Locations under a home directory whose direct children are named after the
/// owning app's bundle identifier. `~/Library/Caches` and `~/Library/Logs` are
/// left out on purpose: the exact cache rules already sweep them.
pub const LEFTOVER_LOCATIONS: &[&str] = &[
    "Library/Application Support",
    "Library/Preferences",
    "Library/Saved Application State",
    "Library/HTTPStorages",
    "Library/WebKit",
    "Library/Cookies",
    "Library/Containers",
    "Library/Application Scripts",
];

/// The one location whose children are commonly named after the app instead.
const NAMED_LOCATION: &str = "Library/Application Support";

/// A folder named after an app counts as abandoned once nothing in it has
/// changed for this long.
pub const STALE_AFTER: Duration = Duration::from_secs(90 * 24 * 60 * 60);

/// How deep the newest modification time is looked for.
const STALE_SEARCH_DEPTH: usize = 2;

/// Where macOS keeps its own frameworks, daemons and tools. Their names
/// identify the Apple-owned folders in `Application Support` (`CallHistoryDB`,
/// `CloudDocs`, `icdd`, …), which carry no bundle identifier.
const SYSTEM_NAME_DIRS: &[&str] = &[
    "/System/Library/PrivateFrameworks",
    "/System/Library/Frameworks",
    "/System/Library/CoreServices",
    "/System/Library/LaunchAgents",
    "/System/Library/LaunchDaemons",
    "/usr/libexec",
    "/usr/sbin",
    "/usr/bin",
];

/// Apple-owned folder names that match no framework or daemon name.
const APPLE_NAMES: &[&str] = &["animoji", "sesstorage"];

/// Name suffixes a bundle identifier can carry in those locations.
const ID_SUFFIXES: &[&str] = &[".plist", ".savedState", ".binarycookies"];

/// First segments accepted as the start of a reverse-DNS identifier.
const ID_ROOTS: &[&str] = &[
    "com", "org", "net", "io", "co", "app", "dev", "me", "ai", "tv", "xyz", "us", "ca", "uk", "de",
    "fr", "nl", "se", "ch", "it", "es", "at", "be", "dk", "fi", "no", "pl", "eu", "au", "jp", "cn",
    "in", "br", "ru",
];

/// Folders holding launch agents, daemons and privileged helpers. A vendor
/// with an entry here still has software on this Mac even without a `.app`.
const SYSTEM_SERVICE_DIRS: &[&str] = &[
    "/Library/LaunchAgents",
    "/Library/LaunchDaemons",
    "/Library/PrivilegedHelperTools",
];

/// What is installed on this Mac, as far as leftovers are concerned.
#[derive(Debug, Default)]
pub struct InstalledApps {
    /// Vendor prefixes (`com.vendor`, lowercased) with software installed.
    vendors: HashSet<String>,
    /// Normalised names installed apps go by: bundle file name, display name,
    /// executable, and each segment of the bundle identifier.
    names: HashSet<String>,
    /// Normalised names of macOS frameworks, daemons and tools.
    system_names: HashSet<String>,
}

impl InstalledApps {
    /// Everything installed for `homes` plus the system-wide locations.
    pub fn discover(homes: &[PathBuf]) -> Self {
        let mut app_dirs = vec![
            PathBuf::from("/Applications"),
            PathBuf::from("/System/Applications"),
        ];
        let mut service_dirs: Vec<PathBuf> =
            SYSTEM_SERVICE_DIRS.iter().map(PathBuf::from).collect();
        for home in homes {
            app_dirs.push(home.join("Applications"));
            service_dirs.push(home.join("Library/LaunchAgents"));
        }
        let system_dirs: Vec<PathBuf> = SYSTEM_NAME_DIRS.iter().map(PathBuf::from).collect();
        Self::from_dirs(&app_dirs, &service_dirs).with_system_names(&system_dirs)
    }

    /// `app_dirs` are searched for `.app` bundles (their `Info.plist` is read);
    /// `service_dirs` contribute the identifiers in their file names.
    pub fn from_dirs(app_dirs: &[PathBuf], service_dirs: &[PathBuf]) -> Self {
        let mut vendors = HashSet::new();
        let mut names = HashSet::new();
        for dir in app_dirs {
            let bundles = walkdir::WalkDir::new(dir)
                .follow_links(false)
                .max_depth(APP_SEARCH_DEPTH)
                .into_iter()
                .filter_entry(|e| e.depth() == 0 || !is_app_bundle(e.path().parent()))
                .filter_map(Result::ok)
                .filter(|e| is_app_bundle(Some(e.path())));
            for bundle in bundles {
                if let Some(stem) = bundle.path().file_stem() {
                    names.insert(normalize_name(&stem.to_string_lossy()));
                }
                let Some(info) = BundleInfo::read(bundle.path()) else {
                    continue;
                };
                names.extend(info.names.iter().map(|n| normalize_name(n)));
                if let Some(id) = &info.id {
                    names.extend(id.split('.').skip(1).map(normalize_name));
                    if let Some(vendor) = vendor_of(id) {
                        vendors.insert(vendor);
                    }
                }
            }
        }
        for dir in service_dirs {
            let Ok(entries) = std::fs::read_dir(dir) else {
                continue;
            };
            for entry in entries.flatten() {
                let name = entry.file_name();
                if let Some(vendor) = bundle_id_in(&name.to_string_lossy()).and_then(vendor_of) {
                    vendors.insert(vendor);
                }
            }
        }
        names.retain(|n| n.len() >= MIN_NAME_LEN);
        InstalledApps {
            vendors,
            names,
            system_names: HashSet::new(),
        }
    }

    /// Add the file names found in `dirs` as macOS-owned names.
    pub fn with_system_names(mut self, dirs: &[PathBuf]) -> Self {
        for dir in dirs {
            let Ok(entries) = std::fs::read_dir(dir) else {
                continue;
            };
            for entry in entries.flatten() {
                let name = entry.file_name();
                let name = name.to_string_lossy();
                let stem = Path::new(name.as_ref())
                    .file_stem()
                    .map(|s| s.to_string_lossy().into_owned())
                    .unwrap_or_default();
                // `com.apple.icdd.plist` → `icdd`
                let last = stem.rsplit('.').next().unwrap_or(&stem);
                for candidate in [normalize_name(&stem), normalize_name(last)] {
                    if candidate.len() >= SIMILAR_MIN_LEN - 1 {
                        self.system_names.insert(candidate);
                    }
                }
            }
        }
        self
    }

    /// No app could be identified at all. Callers must treat this as "unknown"
    /// and report nothing, rather than as "everything is uninstalled".
    pub fn is_empty(&self) -> bool {
        self.vendors.is_empty()
    }

    pub fn has_vendor(&self, vendor: &str) -> bool {
        self.vendors.contains(&vendor.to_ascii_lowercase())
    }

    /// An installed app goes by a name equal to, containing, or contained in
    /// `name`. Errs towards `true`.
    pub fn has_app_named(&self, name: &str) -> bool {
        let name = normalize_name(name);
        name.len() < MIN_NAME_LEN
            || self.names.iter().any(|n| {
                *n == name
                    || (n.len() >= SIMILAR_MIN_LEN - 1
                        && (n.contains(&name) || name.contains(n.as_str())))
            })
    }

    /// `name` is, or starts like, a macOS framework, daemon or tool.
    pub fn is_system_name(&self, name: &str) -> bool {
        let name = normalize_name(name);
        APPLE_NAMES.contains(&name.as_str())
            || self.system_names.iter().any(|s| {
                *s == name
                    || (s.len() >= SIMILAR_MIN_LEN && name.starts_with(s.as_str()))
                    || (name.len() >= SIMILAR_MIN_LEN && s.starts_with(&name))
            })
    }
}

/// Names shorter than this are too ambiguous to compare.
const MIN_NAME_LEN: usize = 3;
/// Minimum length for a prefix / containment match to count.
const SIMILAR_MIN_LEN: usize = 5;

/// Lowercase, ASCII letters and digits only: `"Metro Exodus"` → `metroexodus`.
fn normalize_name(name: &str) -> String {
    name.chars()
        .filter(char::is_ascii_alphanumeric)
        .map(|c| c.to_ascii_lowercase())
        .collect()
}

fn is_app_bundle(path: Option<&Path>) -> bool {
    path.and_then(Path::extension)
        .is_some_and(|ext| ext.eq_ignore_ascii_case("app"))
}

/// The identifying fields of an app's `Info.plist`.
struct BundleInfo {
    id: Option<String>,
    names: Vec<String>,
}

impl BundleInfo {
    fn read(app: &Path) -> Option<Self> {
        let info = plist::Value::from_file(app.join("Contents/Info.plist")).ok()?;
        let info = info.as_dictionary()?;
        let text = |key: &str| info.get(key).and_then(|v| v.as_string()).map(str::to_owned);
        Some(BundleInfo {
            id: text("CFBundleIdentifier"),
            names: ["CFBundleName", "CFBundleDisplayName", "CFBundleExecutable"]
                .into_iter()
                .filter_map(text)
                .collect(),
        })
    }
}

/// The bundle identifier a Library entry is named after, if it has that shape.
pub fn bundle_id_in(name: &str) -> Option<&str> {
    let id = ID_SUFFIXES
        .iter()
        .find_map(|suffix| name.strip_suffix(suffix))
        .unwrap_or(name);
    let mut segments = id.split('.');
    let root = segments.next()?;
    if !ID_ROOTS.contains(&root.to_ascii_lowercase().as_str()) {
        return None;
    }
    let mut rest = 0;
    for segment in segments {
        let valid = !segment.is_empty()
            && segment
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
        if !valid {
            return None;
        }
        rest += 1;
    }
    (rest >= 2).then_some(id)
}

/// `com.vendor.App` → `com.vendor`.
fn vendor_of(id: &str) -> Option<String> {
    let mut segments = id.split('.');
    let root = segments.next()?;
    let vendor = segments.next()?;
    Some(format!("{root}.{vendor}").to_ascii_lowercase())
}

fn is_orphan(name: &str, installed: &InstalledApps) -> bool {
    let Some(id) = bundle_id_in(name) else {
        return false;
    };
    let id_lc = id.to_ascii_lowercase();
    if id_lc.starts_with(SELF_ID) {
        return false;
    }
    match vendor_of(id) {
        Some(vendor) => vendor != "com.apple" && !installed.has_vendor(&vendor),
        None => false,
    }
}

/// A folder named after an app (not an identifier) that nothing installed
/// accounts for and nothing has touched since `stale_before`.
fn is_stale_named_dir(path: &Path, installed: &InstalledApps, stale_before: SystemTime) -> bool {
    let Some(name) = path.file_name().map(|n| n.to_string_lossy().into_owned()) else {
        return false;
    };
    if name.starts_with('.') || bundle_id_in(&name).is_some() {
        return false;
    }
    let is_real_dir = std::fs::symlink_metadata(path).is_ok_and(|m| m.is_dir());
    is_real_dir
        && !installed.has_app_named(&name)
        && !installed.is_system_name(&name)
        && newest_modification(path).is_some_and(|newest| newest < stale_before)
}

/// The most recent modification time of `path` and what is shallowly inside
/// it. `None` when any of it cannot be read — unknown age is not stale.
fn newest_modification(path: &Path) -> Option<SystemTime> {
    let mut newest = std::fs::symlink_metadata(path).ok()?.modified().ok()?;
    for entry in walkdir::WalkDir::new(path)
        .follow_links(false)
        .max_depth(STALE_SEARCH_DEPTH)
    {
        newest = newest.max(entry.ok()?.metadata().ok()?.modified().ok()?);
    }
    Some(newest)
}

#[derive(Debug, PartialEq, Eq)]
pub struct Leftover {
    pub path: PathBuf,
    pub label: &'static str,
}

/// What [`orphans_in_home`] found, plus the locations it could not read.
#[derive(Debug, Default)]
pub struct Leftovers {
    pub items: Vec<Leftover>,
    pub errors: Vec<(PathBuf, io::Error)>,
}

/// Leftover entries under `home`, in a stable order. Returns nothing when
/// `installed` is empty.
pub fn orphans_in_home(home: &Path, installed: &InstalledApps) -> Leftovers {
    orphans_in_home_at(home, installed, SystemTime::now())
}

/// [`orphans_in_home`] with an explicit "now" for the age test.
pub fn orphans_in_home_at(home: &Path, installed: &InstalledApps, now: SystemTime) -> Leftovers {
    let stale_before = now
        .checked_sub(STALE_AFTER)
        .unwrap_or(SystemTime::UNIX_EPOCH);
    let mut out = Leftovers::default();
    if installed.is_empty() {
        return out;
    }
    for location in LEFTOVER_LOCATIONS {
        let base = home.join(location);
        let entries = match std::fs::read_dir(&base) {
            Ok(entries) => entries,
            Err(e) if e.kind() == io::ErrorKind::NotFound => continue,
            Err(e) => {
                out.errors.push((base, e));
                continue;
            }
        };
        let mut found: Vec<Leftover> = entries
            .flatten()
            .filter_map(|entry| {
                let path = entry.path();
                let label = if is_orphan(&entry.file_name().to_string_lossy(), installed) {
                    LABEL
                } else if *location == NAMED_LOCATION
                    && is_stale_named_dir(&path, installed, stale_before)
                {
                    STALE_LABEL
                } else {
                    return None;
                };
                Some(Leftover { path, label })
            })
            .collect();
        found.sort_by(|a, b| a.path.cmp(&b.path));
        out.items.extend(found);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    fn install_app(apps: &Path, name: &str, id: &str) {
        let contents = apps.join(name).join("Contents");
        fs::create_dir_all(&contents).unwrap();
        fs::write(
            contents.join("Info.plist"),
            format!(
                r#"<?xml version="1.0" encoding="UTF-8"?>
<plist version="1.0"><dict><key>CFBundleIdentifier</key><string>{id}</string></dict></plist>"#
            ),
        )
        .unwrap();
    }

    fn names(home: &Path, leftovers: &Leftovers) -> Vec<String> {
        leftovers
            .items
            .iter()
            .map(|l| {
                let rel = l.path.strip_prefix(home).unwrap().to_string_lossy();
                format!("{rel} [{}]", l.label)
            })
            .collect()
    }

    /// A "now" far enough ahead that everything just created is stale.
    fn much_later() -> SystemTime {
        SystemTime::now() + STALE_AFTER + Duration::from_secs(3600)
    }

    #[test]
    fn bundle_id_shape() {
        assert_eq!(bundle_id_in("com.acme.Editor"), Some("com.acme.Editor"));
        assert_eq!(
            bundle_id_in("com.acme.Editor.plist"),
            Some("com.acme.Editor")
        );
        assert_eq!(
            bundle_id_in("org.acme.Tool.savedState"),
            Some("org.acme.Tool")
        );
        // Plain app names, two-segment names and odd shapes are not identifiers.
        assert_eq!(bundle_id_in("Slack"), None);
        assert_eq!(bundle_id_in("com.acme"), None);
        assert_eq!(bundle_id_in("notes.backup.2024"), None);
        assert_eq!(bundle_id_in("com.acme..Editor"), None);
        assert_eq!(bundle_id_in("com.acme.My Editor"), None);
    }

    #[test]
    fn reports_only_entries_whose_vendor_is_gone() {
        let dir = tempdir().unwrap();
        let apps = dir.path().join("Applications");
        let home = dir.path().join("home");
        install_app(&apps, "Editor.app", "com.acme.Editor");
        install_app(&apps.join("Utilities"), "Nested.app", "io.nested.Tool");

        let support = home.join("Library/Application Support");
        let prefs = home.join("Library/Preferences");
        fs::create_dir_all(support.join("com.gone.Player")).unwrap();
        fs::create_dir_all(support.join("com.acme.Helper")).unwrap();
        fs::create_dir_all(support.join("Plain Name")).unwrap();
        fs::create_dir_all(&prefs).unwrap();
        fs::write(prefs.join("com.gone.Player.plist"), b"x").unwrap();
        fs::write(prefs.join("com.acme.Editor.plist"), b"x").unwrap();
        fs::write(prefs.join("io.nested.Tool.plist"), b"x").unwrap();
        fs::write(prefs.join("com.apple.finder.plist"), b"x").unwrap();
        fs::write(prefs.join("com.febritecno.qclean.plist"), b"x").unwrap();

        let installed = InstalledApps::from_dirs(&[apps], &[]);
        let found = orphans_in_home(&home, &installed);

        assert!(found.errors.is_empty());
        assert_eq!(
            names(&home, &found),
            vec![
                format!("Library/Application Support/com.gone.Player [{LABEL}]"),
                format!("Library/Preferences/com.gone.Player.plist [{LABEL}]"),
            ]
        );
    }

    #[test]
    fn a_launch_agent_keeps_its_vendor_installed() {
        let dir = tempdir().unwrap();
        let apps = dir.path().join("Applications");
        let agents = dir.path().join("LaunchAgents");
        let home = dir.path().join("home");
        install_app(&apps, "Editor.app", "com.acme.Editor");
        fs::create_dir_all(&agents).unwrap();
        fs::write(agents.join("com.daemon.updater.plist"), b"x").unwrap();
        let prefs = home.join("Library/Preferences");
        fs::create_dir_all(&prefs).unwrap();
        fs::write(prefs.join("com.daemon.cli.plist"), b"x").unwrap();

        let installed = InstalledApps::from_dirs(&[apps], &[agents]);
        assert!(orphans_in_home(&home, &installed).items.is_empty());
    }

    #[test]
    fn named_folders_need_no_matching_app_no_system_name_and_age() {
        let dir = tempdir().unwrap();
        let apps = dir.path().join("Applications");
        let system = dir.path().join("PrivateFrameworks");
        let home = dir.path().join("home");
        install_app(&apps, "Photo Studio.app", "com.acme.photostudio");
        fs::create_dir_all(system.join("CallHistory.framework")).unwrap();

        let support = home.join("Library/Application Support");
        for name in [
            "Gone Game",
            "PhotoStudio",
            "Acme",
            "CallHistoryDB",
            ".hidden",
        ] {
            fs::create_dir_all(support.join(name).join("data")).unwrap();
        }
        fs::write(support.join("Gone File"), b"x").unwrap();

        let installed = InstalledApps::from_dirs(&[apps], &[]).with_system_names(&[system]);

        // Freshly written: nothing is old enough yet.
        assert!(orphans_in_home(&home, &installed).items.is_empty());

        assert_eq!(
            names(&home, &orphans_in_home_at(&home, &installed, much_later())),
            vec![format!(
                "Library/Application Support/Gone Game [{STALE_LABEL}]"
            )]
        );
    }

    #[test]
    fn nothing_is_reported_when_no_app_can_be_identified() {
        let dir = tempdir().unwrap();
        let home = dir.path().join("home");
        let prefs = home.join("Library/Preferences");
        fs::create_dir_all(&prefs).unwrap();
        fs::write(prefs.join("com.gone.Player.plist"), b"x").unwrap();

        let installed = InstalledApps::from_dirs(&[dir.path().join("missing")], &[]);
        assert!(installed.is_empty());
        assert!(orphans_in_home(&home, &installed).items.is_empty());
    }
}
