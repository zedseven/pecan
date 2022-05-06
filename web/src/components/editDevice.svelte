<script lang="ts">
	// Imports
	import loading from './loading.svelte';
	import responseError from './responseError.svelte';
	import couldntConnect from './couldntConnect.svelte';
	import { fetchDefinitions, selectedLocation } from '../stores';
	import { handleNetworkResponse, Ok, postData, sanitiseObjectMapToArray } from '../util';
	import locationSelector from './locationSelector.svelte';

	// Component Data
	export let deviceId = null;
	let definitions;
	let deviceData = {
		locationId: $selectedLocation, // Default to the selected location for ergonomics
		columnData: {},
	};

	// Fetch the necessary information from the server
	const deviceUrl = '/api/devices/get/';
	let loadingPromise = fetchDefinitions().then(async (definitionsResult) => {
		// If there was an error, return it for processing below
		if (!definitionsResult.ok) return definitionsResult;

		// Store the result
		definitions = definitionsResult.value;

		// Set up the deviceData for binding
		for (const columnDefinition of definitions.columnDefinitions) {
			deviceData.columnData[columnDefinition.id] = {
				columnDefinitionId: columnDefinition.id,
				dataValue: null,
			};
		}

		// Load the device info if there's a device ID to load
		if (!deviceId) return Ok({});
		let deviceResult = await fetch(deviceUrl + deviceId, { method: 'get' }).then(
			handleNetworkResponse,
		);

		// If an error was encountered when fetching the device info, that takes precedence
		if (!deviceResult.ok) return deviceResult;

		// Set the device data based on what was loaded
		deviceData.locationId = deviceResult.value.deviceResults[0].locationId;
		for (const deviceColumnData of deviceResult.value.deviceResults[1]) {
			deviceData.columnData[deviceColumnData.columnDefinitionId] = {
				columnDefinitionId: deviceColumnData.columnDefinitionId,
				dataValue: deviceColumnData.dataValue,
			};
		}

		return Ok({});
	});

	// Submit the data
	const onSubmit = async (event) => {
		event.preventDefault();
		let inputData = {
			locationId: null,
			columnData: [],
		};

		// Validate and sanitise the input data
		if (!deviceData.locationId) {
			alert('You must select the current location of the device.');
			return;
		}
		inputData.locationId = deviceData.locationId;
		inputData.columnData = sanitiseObjectMapToArray(deviceData.columnData);

		// Push it to the server
		const addDeviceUrl = '/api/devices/create';
		const updateDeviceUrl = '/api/devices/update/';
		const url = deviceId ? updateDeviceUrl + deviceId : addDeviceUrl;
		let pushResult = await postData(url, inputData);

		// Redirect if successful
		if (pushResult.ok) {
			window.location = '/';
		}

		console.log(await pushResult.text());
	};
</script>

{#await loadingPromise}
	<svelte:component this={loading} />
{:then loadingResult}
	{#if loadingResult.ok}
		<form on:submit|preventDefault={onSubmit} method="post">
			<table>
				<tr>
					{#if deviceId}
						<th>Device ID</th>
					{/if}
					<th><label for="location">Location</label></th>
					{#each definitions.columnDefinitions as columnDefinition}
						<th><label for="column{columnDefinition.id}">{columnDefinition.name}</label></th>
					{/each}
				</tr>
				<tr>
					{#if deviceId}
						<td class="monospace">{deviceId}</td>
					{/if}
					<td>
						<svelte:component
							this={locationSelector}
							bind:value={deviceData.locationId}
							id="location"
							required={true}
						/>
					</td>
					{#each definitions.columnDefinitions as columnDefinition}
						<td>
							<input
								id="column{columnDefinition.id}"
								type="text"
								placeholder={columnDefinition.name}
								bind:value={deviceData.columnData[columnDefinition.id].dataValue}
							/>
						</td>
					{/each}
				</tr>
			</table>
			{#if deviceId}
				<input type="submit" value="Update" />
			{:else}
				<input type="submit" value="Add" />
			{/if}
		</form>
	{:else}
		<svelte:component this={responseError} error={loadingResult.error} />
	{/if}
{:catch}
	<svelte:component this={couldntConnect} />
{/await}
