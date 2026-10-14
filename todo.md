# TODO

## Done

- [x] Rename MacClean → QClean (app name, crates `qclean` / `qclean-core`, `QCLEAN_EXTRA_SCAN_ROOTS`, storage keys, docs, release scripts)
- [x] App leftovers scan — `~/Library` data of apps that are no longer installed, by bundle identifier and by app name + 90 days unused (`src-tauri/crates/qclean-core/src/leftovers.rs`)
- [x] Linux and Windows support in the engine: per-OS protected paths, cache rules, user-home discovery, disk usage
- [x] UI redesign: neutral palette with one green accent, Outfit + Geist Mono, Hugeicons, simpler dashboard (one Scan button, options behind a disclosure)
- [x] Progress motion: tick meter (disk usage, cleanup progress), radar scan ring, drawn check + burst on completion
- [x] Promo loop `docs/promo/index.html` and `docs/promo/qclean.gif` (`node scripts/promo-gif.mjs` re-renders it)
- [x] New icon (`assets/icon.svg`, all sizes regenerated)
- [x] Animated icon (`assets/icon-animated.svg`, copy in `static/` for the About dialog) and drawn "all clean" art for the empty results state (`AllCleanArt.svelte`)

## Confirm — these were guessed

- [ ] Bundle identifier `com.febritecno.qclean` in `src-tauri/tauri.conf.json` (also `SELF_ID` in `leftovers.rs`)
- [ ] GitHub repo `febritecno/qclean` — README badges, `scripts/updater-manifest.mjs`, `docs/index.html`, updater endpoint
- [ ] Updater `pubkey` in `src-tauri/tauri.conf.json` is still upstream's — generate your own key pair, or updates will fail signature checks (see `docs/signing.md`)
- [ ] `docs/CNAME` was removed (it pointed at upstream's domain) — add your own if the landing page gets a custom domain
- [ ] `.github/FUNDING.yml` still names the upstream author

## Linux and Windows — not yet proven

- [ ] Run the app on a real Linux and a real Windows machine. Only the engine was type-checked for those targets from macOS; nothing was executed there, and the Tauri shell could not be checked for Windows locally (its `ring` dependency needs a Windows C toolchain)
- [ ] Push to GitHub so the new CI jobs run: `core` (engine tests on Ubuntu and Windows) and `tauri-build` (bundles on all three)
- [ ] Before trusting deletion on Windows or Linux, review the protected lists in `safety.rs` on the real machine
- [ ] `.github/workflows/release.yml` still builds the macOS universal DMG only — add Linux (deb/AppImage) and Windows (NSIS) jobs and updater entries
- [ ] Windows Recycle Bin and Linux system caches are not cleaned
- [ ] Home scope walks `~/.vscode/extensions` and similar tool folders, where `node_modules` belongs to a working program (inherited behaviour, all platforms) — consider excluding them

## Repo setup

- [ ] `git init` and first commit (`npm run prepare` installs git hooks and needs a repository)

## App leftovers — follow-ups

- [ ] Apps installed outside `/Applications` and `~/Applications` are not seen, so their data can be listed as a leftover (seen on this Mac: `com.android.Emulator`)
- [ ] Decide whether 90 days is the right age — at 60 days this Mac would also list `vmlx` and `kotlin`
- [ ] "Unused apps" (installed but not opened for months) — not built; would use Spotlight's last-used date

## UI

- [ ] Look at the scanning, cleaning, results and summary screens in the running app — only the dashboard was checked by screenshot
- [x] Retake `docs/screenshot1.png` (now the new scanning screen)
- [ ] Restyle `docs/index.html` — still the old purple look
- [ ] Category chips still use eight colours; decide whether to reduce them to the single accent
