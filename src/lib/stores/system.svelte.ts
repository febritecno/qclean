import { api } from '$lib/api';
import type { PermissionStatus, SystemInfo } from '$lib/types/ipc';

class SystemStore {
	info = $state<SystemInfo | null>(null);
	permissions = $state<PermissionStatus | null>(null);
	/** Accessibility access, which Clean Mode's keyboard lock needs — independent of Full Disk Access. */
	accessibility = $state<boolean | null>(null);
	loading = $state(false);
	error = $state<string | null>(null);

	/** True until the platform is known, so macOS-only UI does not flash away. */
	get isMac(): boolean {
		return (this.info?.platform ?? 'macos') === 'macos';
	}

	/** What to call this machine in copy: "Mac", "PC" or "computer". */
	get device(): string {
		const platform = this.info?.platform ?? 'macos';
		return platform === 'macos' ? 'Mac' : platform === 'windows' ? 'PC' : 'computer';
	}

	/** The platform's file manager. */
	get fileManager(): string {
		const platform = this.info?.platform ?? 'macos';
		return platform === 'macos' ? 'Finder' : platform === 'windows' ? 'Explorer' : 'file manager';
	}

	get home(): string {
		return this.info?.homeDir ?? '';
	}

	async load() {
		this.loading = true;
		this.error = null;
		try {
			const [info, permissions, accessibility] = await Promise.all([
				api.getSystemInfo(),
				api.getPermissionStatus(),
				api.getAccessibilityStatus()
			]);
			this.info = info;
			this.permissions = permissions;
			this.accessibility = accessibility;
		} catch (e) {
			this.error = String(e);
		} finally {
			this.loading = false;
		}
	}

	async refreshPermissions() {
		try {
			const [permissions, accessibility] = await Promise.all([
				api.getPermissionStatus(),
				api.getAccessibilityStatus()
			]);
			this.permissions = permissions;
			this.accessibility = accessibility;
		} catch (e) {
			this.error = String(e);
		}
	}
}

export const system = new SystemStore();
