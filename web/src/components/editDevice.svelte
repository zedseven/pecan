<script lang="ts">
	// Imports
	import loading from './loading.svelte';
	import responseError from './responseError.svelte';
	import couldntConnect from './couldntConnect.svelte';
	import locationSelector from './locationSelector.svelte';
	import { fetchDefinitions, selectedLocation } from '../stores';
	import { getData, Ok, postData, sanitiseObjectMapToArray } from '../util';

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
	let deviceDataDuplicateFlags = {};

	// Fetch the necessary information from the server
	const deviceUrl = '/api/devices/get/';
	let loadingPromise = Promise.all(
		deviceId ? [fetchDefinitions(), getData(deviceUrl + deviceId)] : [fetchDefinitions()],
	).then((combinedResult) => {
		let definitionsResult = combinedResult[0];

		// If there was an error, return it for processing below
		if (!definitionsResult.ok) return definitionsResult;

		// Store the definitions
		definitions = definitionsResult.value;

		// Set up the deviceData for binding
		for (const columnDefinition of definitions.columnDefinitions) {
			deviceData.columnData[columnDefinition[0].id] = {
				columnDefinitionId: columnDefinition[0].id,
				dataValue: null,
			};
			deviceDataDuplicateFlags[columnDefinition[0].id] = false;
		}

		// Parse the device info if there's a device ID
		if (!deviceId) return Ok({});
		let deviceResult = combinedResult[1];

		// If an error was encountered when fetching the device info, that takes precedence
		if (!deviceResult.ok) return deviceResult;

		// Set the device data based on what was loaded
		deviceData.locationId = deviceResult.value.deviceResults[0].locationId;
		for (const deviceColumnData of deviceResult.value.deviceResults[1]) {
			deviceData.columnData[deviceColumnData.columnDefinitionId] = {
				columnDefinitionId: deviceColumnData.columnDefinitionId,
				dataValue: deviceColumnData.dataValue ? deviceColumnData.dataValue : null,
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

		// Validate the input data
		if (!deviceData.locationId) {
			alert('You must select the current location of the device.');
			return;
		}
		for (const columnDefinition of definitions.columnDefinitions) {
			if (deviceDataDuplicateFlags[columnDefinition[0].id]) {
				alert("At least one of the things you've entered is a duplicate, and must be unique.");
				return;
			}
		}

		// Add the existing new component entry to the list if necessary
		addNewComponent();

		// Prepare and sanitise the input data
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

	// Check if the newly-typed value exists in the database, and tell the user if it does
	const ensureValueIsUnique = async (columnId, check) => {
		if (!check) return;

		// Check with the server - this feels gross, but it shouldn't actually be that bad
		const valueExistsUrl = '/api/devices/valueExists/';
		let existsResult = await postData(valueExistsUrl + columnId, {
			deviceId,
			value: deviceData.columnData[columnId].dataValue.trim(),
		});

		// Exit if there was an error
		if (!existsResult.ok) {
			console.log(existsResult.error);
			return;
		}

		console.log(existsResult);

		deviceDataDuplicateFlags[columnId] = existsResult.value.exists;
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
						<th><label for="location" class="block">Location</label></th>
						{#each definitions.columnDefinitions as columnDefinition}
							<th>
								<label for="column{columnDefinition[0].id}" class="block">
									{columnDefinition[0].name}
								</label>
							</th>
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
								{#if columnDefinition[0].exclusivelyPossibleValues}
									<select
										bind:value={deviceData.columnData[columnDefinition[0].id].dataValue}
										id="column{columnDefinition[0].id}"
										class="maxWidth"
										required={columnDefinition[0].notNull}
									>
										<option value={null} disabled="disabled">
											-- {columnDefinition[0].name} --
										</option>
										{#if deviceId && deviceData.columnData[columnDefinition[0].id].dataValue && !columnDefinition[1].some((possibleValue) => possibleValue.value === deviceData.columnData[columnDefinition[0].id].dataValue)}
											<option
												value={deviceData.columnData[columnDefinition[0].id].dataValue}
												disabled="disabled"
											>
												{deviceData.columnData[columnDefinition[0].id].dataValue}
											</option>
										{/if}
										{#each columnDefinition[1] as possibleValue}
											<option value={possibleValue.value}>{possibleValue.value}</option>
										{/each}
									</select>
								{:else}
									<datalist id="column{columnDefinition[0].id}List">
										{#each columnDefinition[1] as possibleValue}
											<option value={possibleValue.value} />
										{/each}
									</datalist>
									{#if deviceDataDuplicateFlags[columnDefinition[0].id]}
										<input
											bind:value={deviceData.columnData[columnDefinition[0].id].dataValue}
											id="column{columnDefinition[0].id}"
											class="maxWidth redBorder"
											type="text"
											required={columnDefinition[0].notNull}
											list="column{columnDefinition[0].id}List"
											placeholder={columnDefinition[0].name}
											title="This value already exists!"
											on:change={ensureValueIsUnique(
												columnDefinition[0].id,
												columnDefinition[0].uniqueValues,
											)}
										/>
									{:else}
										<input
											bind:value={deviceData.columnData[columnDefinition[0].id].dataValue}
											id="column{columnDefinition[0].id}"
											class="maxWidth"
											type="text"
											required={columnDefinition[0].notNull}
											list="column{columnDefinition[0].id}List"
											placeholder={columnDefinition[0].name}
											on:change={ensureValueIsUnique(
												columnDefinition[0].id,
												columnDefinition[0].uniqueValues,
											)}
										/>
									{/if}
								{/if}
							</td>
						{/each}
					</tr>
				</table>
				<br />
				<h2>Components</h2>
				<table>
					<tr>
						<th>Component ID</th>
						<th>Component</th>
					</tr>
					{#each deviceData.components as deviceComponent}
						<tr>
							{#if deviceComponent.componentId}
								<td class="centerContents monospace slightlyLargerFont">
									{deviceId}-{deviceComponent.componentId}
								</td>
							{:else}
								<td class="centerContents monospace noSelect smallerFont">&lt;Not Submitted&gt;</td>
							{/if}
							<td>
								<input
									bind:value={deviceComponent.componentType}
									id="component{deviceComponent.componentId}Type"
									type="text"
									placeholder="Component Type"
								/>
							</td>
						</tr>
					{/each}
					<tr>
						<td><button on:click={addNewComponent} class="maxWidth">Add to List</button></td>
						<td>
							<input
								bind:value={newComponent.componentType}
								id="newComponentType"
								type="text"
								placeholder="Component Type"
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
		{@debug loadingPromise}
		<svelte:component this={couldntConnect} />
	{/await}
</div>

<style>
	h2 {
		margin: 0;
	}

	.redBorder {
		border-color: red;
	}
</style>
