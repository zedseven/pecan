<script lang="ts">
	// Imports
	import loading from './loading.svelte';
	import responseError from './responseError.svelte';
	import couldntConnect from './couldntConnect.svelte';
	import locationSelector from './locationSelector.svelte';
	import { ViewMode } from './editDevice.svelte';
	import barcode from './barcode.svelte';
	import { fetchDefinitions, selectedLocation } from '../stores';
	import { getData, Ok, postData, redirectIfNotLoggedIn, sanitiseObjectMapToArray } from '../util';

	// Component Data
	export let deviceId = null;
	export let viewMode;
	export let isLoading = true;
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
	let locationsMap = {};

	// Fetch the necessary information from the server
	const deviceUrl = '/api/devices/get/';
	isLoading = true;
	let callList = [redirectIfNotLoggedIn(), fetchDefinitions()];
	if (deviceId) callList.push(getData(deviceUrl + deviceId));
	export let loadingPromise = Promise.all(callList)
		.then((combinedResult) => {
			let definitionsResult = combinedResult[1];

			// If there was an error, return it for processing below
			if (!definitionsResult.ok) return definitionsResult;

			// Store the definitions
			definitions = definitionsResult.value;

			// Build the location ID -> location name map
			for (const locationEntry of definitions.locations) {
				locationsMap[locationEntry.id] = locationEntry.name;
			}

			// Set up the deviceData for binding
			for (const columnDefinition of definitions.columnDefinitions) {
				deviceData.columnData[columnDefinition[0].id] = {
					columnDefinitionId: columnDefinition[0].id,
					dataValue: columnDefinition[0].defaultValue,
				};
				deviceDataDuplicateFlags[columnDefinition[0].id] = false;
			}

			// Parse the device info if there's a device ID
			if (!deviceId) return Ok({});
			let deviceResult = combinedResult[2];

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
		})
		.then((result) => {
			if (!result.ok) return result;

			isLoading = false;
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

	// Utility function to display a value as empty if it's null
	const emptyIfNull = (value) => {
		return value != null ? value : '';
	};
</script>

{#await loadingPromise}
	<svelte:component this={loading} />
{:then loadingResult}
	{#if loadingResult.ok}
		<form on:submit|preventDefault={onSubmit} method="post">
			<table id="mainDetails" class:maxWidth={viewMode === ViewMode.Print}>
				{#if deviceId}
					<tr class="noHoverDarken">
						<td colspan="2" class="centerContents monospace largerFont">{deviceId}</td>
					</tr>
					<tr class="noHoverDarken">
						<th colspan="2">
							<svelte:component this={barcode} bind:data={deviceId} />
						</th>
					</tr>
				{/if}
				<tr class="unprintable">
					<td><label for="location" class="block">Location: </label></td>
					<td>
						{#if viewMode}
							<span class="detailEntry">{emptyIfNull(locationsMap[deviceData.locationId])}</span>
						{:else}
							<svelte:component
								this={locationSelector}
								bind:value={deviceData.locationId}
								id="location"
								className="detailEntry detailInput"
								required={true}
							/>
						{/if}
					</td>
				</tr>
				{#each definitions.columnDefinitions as columnDefinition}
					<tr class:unprintable={!columnDefinition[0].showOnLabels}>
						<td>
							<label for="column{columnDefinition[0].id}" class="block">
								{columnDefinition[0].name}:
							</label>
						</td>
						<td>
							{#if viewMode}
								<span class="detailEntry">
									{emptyIfNull(deviceData.columnData[columnDefinition[0].id].dataValue)}
								</span>
							{:else if columnDefinition[0].exclusivelyPossibleValues}
								<select
									bind:value={deviceData.columnData[columnDefinition[0].id].dataValue}
									id="column{columnDefinition[0].id}"
									class="detailEntry detailInput"
									required={columnDefinition[0].notNull}
								>
									<option value={null} disabled={true}>
										-- {columnDefinition[0].name} --
									</option>
									{#if deviceId && deviceData.columnData[columnDefinition[0].id].dataValue && !columnDefinition[1].some((possibleValue) => possibleValue.value === deviceData.columnData[columnDefinition[0].id].dataValue)}
										<option
											value={deviceData.columnData[columnDefinition[0].id].dataValue}
											disabled={true}
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
								<input
									bind:value={deviceData.columnData[columnDefinition[0].id].dataValue}
									id="column{columnDefinition[0].id}"
									class="detailEntry detailInput"
									type="text"
									required={columnDefinition[0].notNull}
									list="column{columnDefinition[0].id}List"
									placeholder={columnDefinition[0].name}
									class:redBorder={deviceDataDuplicateFlags[columnDefinition[0].id]}
									title={deviceDataDuplicateFlags[columnDefinition[0].id]
										? 'This value already exists!'
										: ''}
									on:change={ensureValueIsUnique(
										columnDefinition[0].id,
										columnDefinition[0].uniqueValues,
									)}
								/>
							{/if}
						</td>
					</tr>
				{/each}
			</table>
			<div id="componentDetails">
				<h3>Components</h3>
				<table>
					{#each deviceData.components as deviceComponent}
						<tr>
							{#if deviceComponent.componentId}
								<td>
									<span class="monospace">{deviceId}-{deviceComponent.componentId}</span>:
								</td>
							{:else}
								<td>
									<span class="monospace noSelect smallerFont">&lt;Not Submitted&gt;</span>:
								</td>
							{/if}
							<td>
								{#if viewMode}
									<span class="detailEntry">
										{emptyIfNull(deviceComponent.componentType)}
									</span>
								{:else}
									<input
										bind:value={deviceComponent.componentType}
										id="component{deviceComponent.componentId}Type"
										class="detailEntry detailInput"
										type="text"
										placeholder="Component Type"
									/>
								{/if}
							</td>
						</tr>
					{/each}
					{#if !viewMode}
						<tr>
							<td><button on:click={addNewComponent} class="maxWidth">Add to List</button></td>
							<td>
								<input
									bind:value={newComponent.componentType}
									id="newComponentType"
									class="detailEntry detailInput"
									type="text"
									placeholder="Component Type"
								/>
							</td>
						</tr>
					{/if}
				</table>
			</div>
			{#if !viewMode}
				<br />
				<input type="submit" value={deviceId ? 'Update' : 'Add'} />
			{/if}
		</form>
	{:else}
		<svelte:component this={responseError} error={loadingResult.error} />
	{/if}
{:catch}
	{@debug loadingPromise}
	<svelte:component this={couldntConnect} />
{/await}

<style lang="scss">
	:global {
		/* Global because some sub-components use the classes too */
		.detailEntry {
			float: right;
		}

		.detailInput {
			width: 15em;
		}
	}

	h3 {
		margin: 1em 0 0.2em;
	}

	td {
		padding: 1px 4px;
	}

	.redBorder {
		border-color: red;
	}
</style>
