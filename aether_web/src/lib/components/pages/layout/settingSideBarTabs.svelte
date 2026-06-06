<script lang="ts">
	import Icon from '@iconify/svelte';
	import { settingsTabsStore, selectedTabId, selectedSettingTab, selectTab } from '$lib/systemStore';
</script>

<div class="flex min-h-full flex-col gap-5 p-4">
	<div class="flex flex-wrap gap-2">
		{#each $settingsTabsStore as tab (tab.id)}
			<button
				type="button"
				class={`inline-flex items-center gap-2 rounded-full border px-4 py-2 text-sm transition ${
					$selectedTabId === tab.id
						? 'border-primary-500 bg-primary-50 text-primary-800'
						: 'border-surface-200 bg-surface-100 text-surface-700 hover:border-surface-300'
				}`}
				onclick={() => selectTab(tab.id)}
			>
				<Icon icon={tab.icon.name} font-size={18} />
				{tab.name}
			</button>
		{/each}
	</div>

	<section class="rounded-2xl border border-surface-200 bg-surface-100 p-6">
		<div class="flex flex-col gap-2 sm:flex-row sm:items-center sm:justify-between">
			<div>
				<h2 class="text-xl font-semibold">{$selectedSettingTab?.name}</h2>
				<p class="text-sm text-surface-600">{$selectedSettingTab?.description}</p>
			</div>
			<Icon icon={$selectedSettingTab?.icon.name ?? 'rivet-icons:settings'} font-size={$selectedSettingTab?.icon.size ?? 18} />
			</div>

		<div class="mt-6 text-sm text-surface-700">
			Use the tabs above to switch sections, then update settings in the panel below.
		</div>
	</section>
</div>
