<script lang="ts">
	import { Err, Ok } from '/src/util';

	const url = '/api/devices/recent/10';
	let resultData = fetch(url, { method: 'get' }).then(async (response) => {
		if (!response.ok) {
			return Err({
				status: response.status,
				statusText: response.statusText,
				message: await response.text(),
			});
		}
		return Ok(await response.json());
	});
</script>

{#await resultData}
	<p>Loading...</p>
{:then data}
	{#if data.ok}
		<table>
			<tr>
				<th>Device ID</th>
				<th>Location</th>
				<th>Last Edit</th>
				{#each data.value.columnDefinitions as columnDefinition}
					<th>{columnDefinition.name}</th>
				{/each}
			</tr>
			{#each data.value.deviceResults as deviceResult}
				<tr>
					<td style="font-family: monospace, monospace;">{deviceResult[0].deviceId}</td>
					<td>{deviceResult[0].location}</td>
					<td>{deviceResult[0].lastUpdated}</td>
					{#each deviceResult[1] as columnValue}
						<td>{columnValue.dataValue}</td>
					{/each}
				</tr>
			{/each}
		</table>
	{:else}
		<p>
			Received an error from the server: {data.error.status}
			{data.error.statusText}: {data.error.message}
		</p>
	{/if}
{:catch}
	<p>Didn't receive a response from the server. Perhaps check your Internet connection?</p>
{/await}
