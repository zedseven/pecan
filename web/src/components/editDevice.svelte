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
		components: [],
	};
	let newComponent = {
		componentId: null,
		componentType: '',
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
		for (const deviceComponent of deviceResult.value.deviceComponents) {
			deviceData.components.push({
				componentId: deviceComponent.componentId,
				componentType: deviceComponent.componentType,
			});
		}

		return Ok({});
	});

	// Submit the data
	const onSubmit = async (event) => {
		event.preventDefault();
		let inputData = {
			locationId: null,
			columnData: [],
			components: [],
		};

		// Add the existing new component entry to the list if necessary
		addNewComponent();

		// Validate and sanitise the input data
		if (!deviceData.locationId) {
			alert('You must select the current location of the device.');
			return;
		}
		inputData.locationId = deviceData.locationId;
		inputData.columnData = sanitiseObjectMapToArray(deviceData.columnData);
		inputData.components = deviceData.components;

		// Push it to the server
		const addDeviceUrl = '/api/devices/create';
		const updateDeviceUrl = '/api/devices/update/';
		const url = deviceId ? updateDeviceUrl + deviceId : addDeviceUrl;
		let pushResult = await postData(url, inputData);

		console.log(pushResult);

		// Redirect/refresh if successful
		if (pushResult.ok) {
			window.location = '/edit/' + pushResult.value.deviceId;
		}
	};

	// Add a new component to the list
	const addNewComponent = (event = undefined) => {
		if (event) event.preventDefault();
		if (!newComponent.componentType) return;
		deviceData.components = [...deviceData.components, Object.assign({}, newComponent)];
		newComponent.componentType = '';
	};
</script>

<div id="content">
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
							<td class="centerContents monospace slightlyLargerFont">{deviceId}</td>
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
				<br />
				<h2>Components</h2>
				<table>
					<tr>
						<th>ID</th>
						<th>Component</th>
					</tr>
					{#each deviceData.components as deviceComponent}
						<tr>
							{#if deviceComponent.componentId}
								<td class="centerContents monospace slightlyLargerFont">
									{deviceId}-{deviceComponent.componentId}
								</td>
							{:else}
								<td class="centerContents monospace">&lt;Not Submitted&gt;</td>
							{/if}
							<td>
								<input
									id="component{deviceComponent.componentId}Type"
									type="text"
									placeholder="Component Type"
									bind:value={deviceComponent.componentType}
								/>
							</td>
						</tr>
					{/each}
					<tr>
						<td><button on:click={addNewComponent} class="maxWidth">Add to List</button></td>
						<td>
							<input
								id="newComponentType"
								type="text"
								placeholder="Component Type"
								bind:value={newComponent.componentType}
							/>
						</td>
					</tr>
				</table>
				<br />
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
</div>
