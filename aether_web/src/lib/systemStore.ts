import { writable, derived, type Readable, type Writable } from 'svelte/store';

export type IconType = 'iconify' | 'lucide';

export interface SettingIcon {
	size: number;
	name: string;
}

export interface SettingTab {
	id: string;
	name: string;
	description?: string;
	icon: SettingIcon;
	iconType: IconType;
}

export const settingsTabs: SettingTab[] = [
	{
		id: 'general',
		name: 'General',
		description: 'General system settings',
		icon: {
			size: 20,
			name: 'rivet-icons:settings'
		},
		iconType: 'iconify'
	},
	{
		id: 'appearance',
		name: 'Appearance',
		description: 'Theme and display preferences',
		icon: {
			size: 20,
			name: 'rivet-icons:palette'
		},
		iconType: 'iconify'
	},
	{
		id: 'notifications',
		name: 'Notifications',
		description: 'Notification settings',
		icon: {
			size: 20,
			name: 'rivet-icons:bell'
		},
		iconType: 'iconify'
	},
	{
		id: 'privacy',
		name: 'Privacy & Security',
		description: 'Privacy and security options',
		icon: {
			size: 20,
			name: 'rivet-icons:lock'
		},
		iconType: 'iconify'
	},
	{
		id: 'account',
		name: 'Account',
		description: 'Account management',
		icon: {
			size: 20,
			name: 'rivet-icons:user'
		},
		iconType: 'iconify'
	}
];

export const settingsTabsStore: Writable<SettingTab[]> = writable(settingsTabs);
export const selectedTabId: Writable<string> = writable(settingsTabs[0]?.id ?? 'general');

export const selectedSettingTab: Readable<SettingTab | undefined> = derived(
	[settingsTabsStore, selectedTabId],
	([$tabs, $selectedId]) => $tabs.find((tab) => tab.id === $selectedId) ?? $tabs[0]
);

export function selectTab(tabId: string) {
	selectedTabId.set(tabId);
}

export const settingsSideBarsStore: Readable<SettingTab[]> = settingsTabsStore;
