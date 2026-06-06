<script lang="ts">
	import Icon from '@iconify/svelte';
	import { Navigation } from '@skeletonlabs/skeleton-svelte';
	import {
		settingsTabsStore,
		selectedTabId,
		selectedSettingTab,
		selectTab
	} from '$lib/systemStore';
</script>

<Navigation layout="sidebar" class="grid grid-rows-[auto_1fr_auto] gap-4">
	<Navigation.Header class="just flex items-center justify-center gap-2">
		<Icon icon="material-symbols:settings" font-size="45" />
		<span class="text-2xl font-semibold">System Settings</span>
	</Navigation.Header>

	<Navigation.Content>
		{#each $settingsTabsStore as settingTab (settingTab.id)}
			<Navigation.Group class="flex items-center gap-2">
				<Navigation.TriggerAnchor
					href={'#' + settingTab.id}
					class={`flex w-full items-center gap-3 rounded-md px-3 py-2 transition hover:bg-surface-100 ${
						$selectedTabId === settingTab.id ? 'bg-surface-100 font-semibold' : ''
					}`}
					aria-current={$selectedTabId === settingTab.id ? 'page' : undefined}
					on:click={(event) => {
						event.preventDefault();
						selectTab(settingTab.id);
					}}
				>
					{#if settingTab.iconType === 'iconify'}
						<Icon icon={settingTab.icon.name} font-size={settingTab.icon.size} />
					{:else}
						<span class="inline-block h-5 w-5" aria-hidden="true"></span>
					{/if}
					<Navigation.TriggerText>{settingTab.name}</Navigation.TriggerText>
				</Navigation.TriggerAnchor>
			</Navigation.Group>
		{/each}
	</Navigation.Content>

	<div class="px-3 pb-4">
		<h3 class="text-sm font-semibold">Current section</h3>
		<p class="text-sm text-surface-600">{$selectedSettingTab?.description ?? 'Choose a section to view details.'}</p>
	</div>
</Navigation>
