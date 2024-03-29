<script lang="ts">
	// Imports
	import loading from './loading.svelte';
	import responseError from './responseError.svelte';
	import couldntConnect from './couldntConnect.svelte';
	import locationSelector from './locationSelector.svelte';
	import { ViewMode } from './editDevice.svelte';
	import deviceChanges from './deviceChanges.svelte';
	import barcode from './barcode.svelte';
	import { fetchDefinitions, selectedLocation } from '../stores';
	import {
		getData,
		Ok,
		postData,
		redirectIfNotLoggedIn,
		sanitiseObjectMapToArray,
		emptyIfNull,
	} from '../util';
	import multiStageButton from './multiStageButton.svelte';

	// Component Data
	export let deviceId = null;
	export let viewMode;
	export let isLoading = true;
	let definitions;
	let deviceData = {
		deleted: false,
		locationId: $selectedLocation, // Default to the selected location for ergonomics
		columnData: {},
		components: [],
		attachments: [],
		changes: [],
	};
	let newComponent = {
		componentType: '',
	};
	let newAttachment = {
		fileElement: null,
		description: '',
	};
	let deviceDataEmptyFlags = {};
	let deviceDataDuplicateFlags = {};
	let newAttachmentIsTooLarge = false;
	let locationsMap = {};
	let columnDefinitionsMap = {};

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

			// Set up the deviceData for binding, prepare the duplicate tracker, and build the column definition ID -> column name map
			for (const columnDefinition of definitions.columnDefinitions) {
				deviceData.columnData[columnDefinition[0].id] = {
					columnDefinitionId: columnDefinition[0].id,
					dataValue: columnDefinition[0].defaultValue,
				};

				deviceDataEmptyFlags[columnDefinition[0].id] = false;
				deviceDataDuplicateFlags[columnDefinition[0].id] = false;

				columnDefinitionsMap[columnDefinition[0].id] = columnDefinition;
			}

			// Parse the device info if there's a device ID
			if (!deviceId) return Ok({});
			let deviceResult = combinedResult[2];

			// If an error was encountered when fetching the device info, that takes precedence
			if (!deviceResult.ok) return deviceResult;

			// Set the device data based on what was loaded
			deviceData.deleted = deviceResult.value.deviceResults[0].deleted;
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
					deleted: deviceComponent.deleted,
					componentType: deviceComponent.componentType,
				});
			}
			for (const deviceAttachment of deviceResult.value.deviceAttachments) {
				deviceData.attachments.push({
					attachmentId: deviceAttachment.attachmentId,
					deleted: deviceAttachment.deleted,
					description: deviceAttachment.description,
					fileName: deviceAttachment.fileName, // Not needed for submission, but needed for display
				});
			}
			for (const deviceChange of deviceResult.value.deviceChanges) {
				let change = JSON.parse(deviceChange.change);
				// Sort the device data changes based on the column definition ordering
				if (change.deviceData != null && change.deviceData.length > 0) {
					change.deviceData.sort((a, b) => {
						// Get the column definitions for both items
						const columnDefA = columnDefinitionsMap[a.columnDefinitionId];
						const columnDefB = columnDefinitionsMap[b.columnDefinitionId];

						// Order by the ordering key
						const orderingKeyDifference = columnDefA[0].orderingKey - columnDefB[0].orderingKey;
						if (orderingKeyDifference != 0) {
							return orderingKeyDifference;
						}

						// Then by the column definition ID (for consistency)
						return columnDefA[0].id - columnDefB[0].id;
					});
				}

				deviceData.changes.push({
					timestamp: deviceChange.timestamp,
					doneAutomatically: deviceChange.doneAutomatically,
					user: deviceChange.user,
					change: change,
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
			attachments: [],
		};

		// Validate the input data
		if (!deviceData.locationId) {
			alert('You must select the current location of the device.');
			return;
		}
		for (const columnDefinition of definitions.columnDefinitions) {
			ensureValueIsNotEmpty(columnDefinition[0].id, columnDefinition[0].notNull);
			if (deviceDataEmptyFlags[columnDefinition[0].id]) {
				alert("At least one of the fields you've tried to submit is empty, and must have a value.");
				return;
			}
			if (deviceDataDuplicateFlags[columnDefinition[0].id]) {
				alert("At least one of the things you've entered is a duplicate, and must be unique.");
				return;
			}
		}

		// Add the existing new component entry to the list if necessary
		addNewComponent();
		await addNewAttachment();

		// If the new attachment that was attempted to be added was too large, fail the update so that it can be addressed
		if (newAttachmentIsTooLarge) {
			alert('The new attachment is too large to be uploaded.');
			return;
		}

		// Prepare and sanitise the input data
		inputData.locationId = deviceData.locationId;
		inputData.columnData = sanitiseObjectMapToArray(deviceData.columnData);
		inputData.components = deviceData.components;
		inputData.attachments = deviceData.attachments;

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
		// Cancel any other events caused by a button click
		if (event) event.preventDefault();

		// Exit if there's nothing to do
		if (!newComponent.componentType) return;

		// Add the new data to the list
		deviceData.components = [
			...deviceData.components,
			Object.assign(
				{},
				{ componentId: null, deleted: false, componentType: newComponent.componentType },
			),
		];

		// Clear the inputs
		newComponent.componentType = '';
	};

	// Add a new attachment to the list
	const addNewAttachment = async (event = undefined) => {
		// Cancel any other events caused by a button click
		if (event) event.preventDefault();

		// Ensure the file size is below the configured limit
		checkNewAttachmentSize();
		if (newAttachmentIsTooLarge) return;

		// Exit if there's nothing to do
		if (
			!newAttachment.fileElement.files ||
			newAttachment.fileElement.files.length < 1 ||
			!newAttachment.fileElement.files[0]
		)
			return;

		// Encode the file data
		let newFileData = await fileToBase64(newAttachment.fileElement.files[0]);

		// Add the new data to the list
		deviceData.attachments = [
			...deviceData.attachments,
			Object.assign(
				{},
				{
					description: newAttachment.description,
					fileName: newAttachment.fileElement.files[0].name,
					fileData: newFileData,
				},
			),
		];

		// Clear the inputs - the way this works is stupid
		newAttachment.fileElement.value = null;
		newAttachment.description = '';
	};

	const clearInputValue = (event) => {
		event.target.value = null;
	};

	// Check if the newly-changed value is empty, and show an error if it does
	const ensureValueIsNotEmpty = (columnId: number, check: boolean) => {
		if (!check) return;

		deviceDataEmptyFlags[columnId] =
			deviceData.columnData[columnId].dataValue == null ||
			deviceData.columnData[columnId].dataValue.trim().length <= 0;
	};

	// Check if the newly-typed value exists in the database, and tell the user if it does
	const ensureValueIsUnique = async (columnId: number, check: boolean) => {
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

	// Check if the newly-selected file is too large
	const checkNewAttachmentSize = () => {
		// Exit if there's nothing to do
		if (
			!newAttachment.fileElement.files ||
			newAttachment.fileElement.files.length < 1 ||
			!newAttachment.fileElement.files[0]
		) {
			newAttachmentIsTooLarge = false;
			return;
		}

		// Ensure the file size is below the configured limit
		newAttachmentIsTooLarge =
			newAttachment.fileElement.files[0].size > definitions.maxAttachmentSize;
	};

	const fileToBase64 = (file) =>
		new Promise((resolve, reject) => {
			if (file.size <= 0) {
				// When given an empty file, Chrome simply returns a value of `data:`
				// Firefox returns a proper value with a mime type: `data:text/plain;base64,`
				console.log('Attachment file is empty, so nothing will be read from disk.');
				resolve('');
				return;
			}

			const reader = new FileReader();
			reader.readAsDataURL(file);
			reader.onload = () => {
				// Remove the data URI beginning (`data:mime;base64,`)
				// console.log(reader.result);
				let resultParts = reader.result.split(',');
				if (resultParts.length == 2) {
					resolve(resultParts[1]);
				} else {
					reject();
				}
			};
			reader.onerror = (error) => reject(error);
		});
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
									class:redBorder={deviceDataEmptyFlags[columnDefinition[0].id]}
									title={deviceDataEmptyFlags[columnDefinition[0].id]
										? 'This value cannot be empty!'
										: ''}
									on:change={ensureValueIsNotEmpty(
										columnDefinition[0].id,
										columnDefinition[0].notNull,
									)}
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
									class:redBorder={deviceDataEmptyFlags[columnDefinition[0].id] ||
										deviceDataDuplicateFlags[columnDefinition[0].id]}
									title={deviceDataEmptyFlags[columnDefinition[0].id]
										? 'This value cannot be empty!'
										: deviceDataDuplicateFlags[columnDefinition[0].id]
										? 'This value already exists!'
										: ''}
									on:change={() => {
										ensureValueIsNotEmpty(columnDefinition[0].id, columnDefinition[0].notNull);
										ensureValueIsUnique(columnDefinition[0].id, columnDefinition[0].uniqueValues);
									}}
								/>
							{/if}
						</td>
					</tr>
				{/each}
			</table>
			{#if !viewMode || deviceData.components.length > 0}
				<div id="componentDetails">
					<h3>Components</h3>
					<table>
						{#each deviceData.components as deviceComponent}
							<tr>
								{#if deviceComponent.componentId}
									<td class:strikethrough={deviceComponent.deleted}>
										<span class="monospace">{deviceId}-{deviceComponent.componentId}</span><span
											class="noSelect">:</span
										>
									</td>
								{:else}
									<td>
										<span class="italicised noSelect smallerFont">&lt;Not Submitted&gt;</span>
									</td>
								{/if}
								<td>
									{#if viewMode}
										{emptyIfNull(deviceComponent.componentType)}
									{:else}
										<input
											bind:value={deviceComponent.componentType}
											id="component{deviceComponent.componentId}Type"
											class="detailInput"
											type="text"
											disabled={deviceComponent.deleted}
											placeholder="Component Type"
										/>
									{/if}
								</td>
								{#if !viewMode}
									<td>
										{#if deviceComponent.componentId}
											{#if deviceComponent.deleted}
												<button
													on:click={() => (deviceComponent.deleted = false)}
													class="maxWidth"
													style="width: 5em;">Undo</button
												>
											{:else}
												<svelte:component
													this={multiStageButton}
													id="component{deviceComponent.componentId}DeleteButton"
													clickedFunction={() => (deviceComponent.deleted = true)}
													defaultText="Delete"
													primedText="Sure?"
													loadingText="Loading"
													primeTimeout={2000}
													className="maxWidth"
													width="5em"
												/>
											{/if}
										{/if}
									</td>
								{/if}
							</tr>
						{/each}
						{#if !viewMode}
							<tr>
								<td><button on:click={addNewComponent} class="maxWidth">Add to List</button></td>
								<td>
									<input
										bind:value={newComponent.componentType}
										id="newComponentType"
										class="detailInput"
										type="text"
										placeholder="Component Type"
									/>
								</td>
								{#if !viewMode}
									<td />
								{/if}
							</tr>
						{/if}
					</table>
				</div>
			{/if}
			{#if !viewMode || deviceData.attachments.length > 0}
				<div id="attachmentDetails">
					<h3>File Attachments</h3>
					<table>
						{#each deviceData.attachments as deviceAttachment}
							<tr>
								{#if deviceAttachment.attachmentId}
									<td class:strikethrough={deviceAttachment.deleted}>
										<span class="monospace">{deviceId}-{deviceAttachment.attachmentId}</span><span
											class="noSelect">:</span
										>
									</td>
								{:else}
									<td>
										<span class="italicised noSelect smallerFont">&lt;Not Submitted&gt;</span>
									</td>
								{/if}
								<td class:strikethrough={deviceAttachment.deleted}>
									{#if viewMode && deviceAttachment.attachmentId}
										<a
											href="/api/devices/attachment/{deviceId}/{deviceAttachment.attachmentId}"
											target="_blank"
											title="Click to Download"
										>
											<span class="monospace">
												{emptyIfNull(deviceAttachment.fileName)}
											</span>
										</a>
									{:else}
										<span class="monospace">
											{emptyIfNull(deviceAttachment.fileName)}
										</span>
									{/if}
								</td>
								<td>
									{#if viewMode}
										{emptyIfNull(deviceAttachment.description)}
									{:else}
										<input
											bind:value={deviceAttachment.description}
											id="attachment{deviceAttachment.attachmentId}Description"
											class="detailEntry detailInput"
											type="text"
											disabled={deviceAttachment.deleted}
											placeholder="Attachment Description"
										/>
									{/if}
								</td>
								{#if !viewMode}
									<td>
										{#if deviceAttachment.attachmentId}
											{#if deviceAttachment.deleted}
												<button
													on:click={() => (deviceAttachment.deleted = false)}
													class="maxWidth"
													style="width: 5em;">Undo</button
												>
											{:else}
												<svelte:component
													this={multiStageButton}
													id="component{deviceAttachment.attachmentId}DeleteButton"
													clickedFunction={() => (deviceAttachment.deleted = true)}
													defaultText="Delete"
													primedText="Sure?"
													loadingText="Loading"
													primeTimeout={2000}
													className="maxWidth"
													width="5em"
												/>
											{/if}
										{/if}
									</td>
								{/if}
							</tr>
						{/each}
						{#if !viewMode}
							<tr>
								<td><button on:click={addNewAttachment} class="maxWidth">Add to List</button></td>
								<td>
									<input
										bind:this={newAttachment.fileElement}
										id="newAttachmentFile"
										class="detailInput"
										type="file"
										placeholder="Attachment File"
										class:redBorder={newAttachmentIsTooLarge}
										class:noRedBorder={!newAttachmentIsTooLarge}
										title={newAttachmentIsTooLarge ? 'This file is too large!' : ''}
										on:click={clearInputValue}
										on:change={checkNewAttachmentSize}
									/>
								</td>
								<td>
									<input
										bind:value={newAttachment.description}
										id="newAttachmentDescription"
										class="detailInput"
										type="text"
										placeholder="Attachment Description"
									/>
								</td>
								{#if !viewMode}
									<td />
								{/if}
							</tr>
						{/if}
					</table>
				</div>
			{/if}
			{#if !viewMode}
				<br />
				<input type="submit" value={deviceId ? 'Update' : 'Add'} />
			{/if}
			{#if deviceId}
				<svelte:component
					this={deviceChanges}
					changeList={deviceData.changes}
					{deviceId}
					{locationsMap}
					{columnDefinitionsMap}
				/>
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

		h3 {
			margin: 1em 0 0.2em;
		}
	}

	td {
		padding: 1px 4px;
	}

	input[type='file'] {
		min-width: 100%;
	}
</style>
