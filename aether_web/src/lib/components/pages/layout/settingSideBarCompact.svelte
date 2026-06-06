<script lang="ts">
	import Icon from '@iconify/svelte';
	import { settingsTabsStore, selectedTabId, selectedSettingTab, selectTab } from '$lib/systemStore';
</script>

<aside class="flex h-full flex-col gap-4 border-r border-surface-200 bg-surface-50 p-4">
	<div class="flex items-center gap-3">
		<Icon icon="material-symbols:settings" font-size="36" />
		<div>
			<p class="text-base font-semibold">Settings</p>
			<p class="text-sm text-surface-600">Manage app preferences</p>
		</div>
	</div>

	<nav class="space-y-2">
		{#each $settingsTabsStore as tab (tab.id)}
			<button
				type="button"
				class={`flex w-full items-center gap-3 rounded-lg px-3 py-2 text-left transition hover:bg-surface-100 ${
					$selectedTabId === tab.id ? 'bg-surface-100 font-semibold shadow-sm' : 'text-surface-700'
				}`}
				on:click={() => selectTab(tab.id)}
			>
				<Icon icon={tab.icon.name} font-size={tab.icon.size} />
				<span>{tab.name}</span>
			</button>
		{/each}
	</nav>

	<div class="mt-auto rounded-2xl bg-surface-100 p-4 text-sm text-surface-700">
		<p class="font-semibold">Selected section</p>
		<p class="mt-2 text-surface-600">{$selectedSettingTab?.name}</p>
		<p class="mt-1 text-xs text-surface-500">{$selectedSettingTab?.description}</p>
	</div>
</aside>
