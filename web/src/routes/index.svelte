<script lang="ts">
	import loading from '../components/loading.svelte';
	import responseError from '../components/responseError.svelte';
	import couldntConnect from '../components/couldntConnect.svelte';
	import { handleNetworkResponse } from '../util';

	const url = '/api/devices/recent/10';
	let resultData = fetch(url, { method: 'get' }).then(handleNetworkResponse);
</script>

{#await resultData}
	<svelte:component this={loading} />
{:then data}
	{#if data.ok}
		<table>
			<tr>
				<th>Device ID</th>
				<th>Location</th>
				<th>Last Updated</th>
				{#each data.value.columnDefinitions as columnDefinition}
					<th>{columnDefinition.name}</th>
				{/each}
				<th />
			</tr>
			{#each data.value.deviceResults as deviceResult}
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
		<svelte:component this={responseError} error={data.error} />
	{/if}
{:catch}
	<svelte:component this={couldntConnect} />
{/await}
