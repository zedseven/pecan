<script lang="ts">
	// Imports
	import { appName } from '../constants';
	import { fetchDefinitions } from '../stores';

	// Component Data
	export let value = null;
	export let id = null;
	export let className = null;
	export let required = false;
	export let emptyValueLabel = '-- Location --';
	export let disableEmptyValue = true;
	let definitions;

	// Load the definitions
	let loadingPromise = fetchDefinitions().then(async (definitionsResult) => {
		// If there was an error, return it for processing below
		if (!definitionsResult.ok) return definitionsResult;

		// Store the result
		definitions = definitionsResult.value;

		return definitionsResult;
	});
</script>

{#await loadingPromise then loadingResult}
	{#if loadingResult.ok}
		<select
			bind:value
			{id}
			class={className}
			{required}
			title="If you don't see the person you're looking for in the list, it's probably because they've never logged into {appName}. Once they log in for the first time, they'll appear here."
		>
			<option value={null} disabled={disableEmptyValue}>{emptyValueLabel}</option>
			{#each definitions.locations as location}
				<option value={location.id}>{location.name}</option>
			{/each}
		</select>
	{/if}
{/await}
