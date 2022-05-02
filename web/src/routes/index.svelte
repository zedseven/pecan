<script lang="ts">
	// Imports
	import navBar from '../components/navBar.svelte';
	import loading from '../components/loading.svelte';
	import responseError from '../components/responseError.svelte';
	import couldntConnect from '../components/couldntConnect.svelte';
	import { handleNetworkResponse } from '../util';

	// Load the devices
	const url = '/api/devices/recent/10';
	let loadingPromise = fetch(url, { method: 'get' }).then(handleNetworkResponse);
</script>

<svelte:head>
	<title>Devices</title>
</svelte:head>

<svelte:component this={navBar} />

{#await loadingPromise}
	<svelte:component this={loading} />
{:then loadingResult}
	{#if loadingResult.ok}
		<table>
			<tr class="headerRow">
				<th>Device ID</th>
				<th>Location</th>
				<th>Last Updated</th>
				{#each loadingResult.value.columnDefinitions as columnDefinition}
					<th>{columnDefinition.name}</th>
				{/each}
				<th />
			</tr>
			{#each loadingResult.value.deviceResults as deviceResult}
				<tr>
					<td class="monospace">{deviceResult[0].deviceId}</td>
					<td>{deviceResult[0].location}</td>
					<td>{deviceResult[0].lastUpdated}</td>
					{#each deviceResult[1] as columnValue}
						<td>{columnValue.dataValue}</td>
					{/each}
					<td><a href="/edit/{deviceResult[0].deviceId}">Edit</a></td>
				</tr>
			{/each}
		</table>
	{:else}
		<svelte:component this={responseError} error={loadingResult.error} />
	{/if}
{:catch}
	<svelte:component this={couldntConnect} />
{/await}

<style>
	tr:not(.headerRow):hover {
		background-color: #dddddd;
	}
</style>
