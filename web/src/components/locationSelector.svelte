<script lang="ts">
	// Imports
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
		<select {id} class={className} {required} bind:value>
			<option value={null} disabled={disableEmptyValue}>{emptyValueLabel}</option>
			{#each definitions.locations as location}
				<option value={location.id}>{location.name}</option>
			{/each}
		</select>
	{/if}
{/await}
