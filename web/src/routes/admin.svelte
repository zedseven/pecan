<script lang="ts">
	// Imports
	import loading from '../components/loading.svelte';
	import responseError from '../components/responseError.svelte';
	import couldntConnect from '../components/couldntConnect.svelte';
	import navBar from '../components/navBar.svelte';
	import { appNameCased } from '../constants';
	import { fetchDefinitions } from '../stores';
	import { Ok } from '../util';

	// Component Data
	let definitions;

	// Load the necessary data
	let loadingPromise = fetchDefinitions().then((definitionsResult) => {
		// If there was an error, return it for processing below
		if (!definitionsResult.ok) return definitionsResult;

		// Store the definitions
		definitions = definitionsResult.value;

		return Ok({});
	});
</script>

<svelte:head>
	<title>Admin - {appNameCased}</title>
</svelte:head>

<svelte:component this={navBar} />

<div id="content">
	{#await loadingPromise}
		<svelte:component this={loading} />
	{:then loadingResult}
		{#if loadingResult.ok}
			<h2>Column Definitions</h2>
			<br />
			<h2>Locations</h2>
		{:else}
			<svelte:component this={responseError} error={loadingResult.error} />
		{/if}
	{:catch}
		{@debug loadingPromise}
		<svelte:component this={couldntConnect} />
	{/await}
</div>

<style lang="scss">
	h2 {
		margin: 0;
	}
</style>
