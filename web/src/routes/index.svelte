<script lang="ts">
	// Imports
	import navBar from '../components/navBar.svelte';
	import loading from '../components/loading.svelte';
	import responseError from '../components/responseError.svelte';
	import couldntConnect from '../components/couldntConnect.svelte';
	import searchBar from '../components/searchBar.svelte';
	import datetime from '../components/datetime.svelte';
	import checkoutButton from '../components/checkoutButton.svelte';
	import locationSelector from '../components/locationSelector.svelte';
	import { fetchDefinitions } from '../stores';
	import { Ok, postData, sanitiseObjectMapToArray } from '../util';

	// Component Data
	let definitions;
	let deviceResults;
	let searchData = {
		deviceId: '',
		locationId: null,
		columnData: {},
	};

	// Handle search queries
	const sendSearch = async (data = null) => {
		let inputData = {
			deviceId: '',
			locationId: null,
			columnData: [],
		};

		if (data) {
			inputData.deviceId = data.deviceId;
			inputData.locationId = data.locationId ? data.locationId : null;
			inputData.columnData = sanitiseObjectMapToArray(data.columnData);
		}

		const searchUrl = '/api/devices/search';
		return postData(searchUrl, inputData).then(async (searchResult) => {
			// If there was an error, return it for processing below
			if (!searchResult.ok) return searchResult;

			// Store the results
			deviceResults = searchResult.value.deviceResults;

			return searchResult;
		});
	};

	const onSearch = async (event) => {
		event.preventDefault();

		loadingPromise = sendSearch(searchData);
	};

	// Load the devices
	let loadingPromise = Promise.all([fetchDefinitions(), sendSearch(null)]).then(
		(combinedResult) => {
			let definitionsResult = combinedResult[0];
			let loadResult = combinedResult[1];

			// If there was an error with either query, return it for processing below
			if (!definitionsResult.ok) return definitionsResult;
			if (!loadResult.ok) return loadResult;

			// Store the definitions
			definitions = definitionsResult.value;

			// Set up the searchData for binding
			for (const columnDefinition of definitions.columnDefinitions) {
				searchData.columnData[columnDefinition[0].id] = {
					columnDefinitionId: columnDefinition[0].id,
					dataValue: null,
				};
			}

			return Ok({});
		},
	);
</script>

<svelte:head>
	<title>Devices</title>
</svelte:head>

<svelte:component this={navBar} />

<div id="content">
	<h3 id="newDeviceLink"><a href="/edit">Add a New Device</a></h3>
	{#await loadingPromise}
		<svelte:component this={loading} />
	{:then loadingResult}
		{#if loadingResult.ok}
			<form on:submit|preventDefault={onSearch} method="post">
				<table>
					<tr class="headerRow">
						<th colspan="2">Last-Updated</th>
						<th><label for="filterDeviceId" class="block">Device ID</label></th>
						<th><label for="filterLocation" class="block">Location</label></th>
						{#each definitions.columnDefinitions as columnDefinition}
							{#if columnDefinition[0].showInMainPage}
								<th>
									<label for="filterColumn{columnDefinition[0].id}" class="block">
										{columnDefinition[0].name}
									</label>
								</th>
							{/if}
						{/each}
					</tr>
					<tr class="headerRow">
						<td colspan="2"><input type="submit" id="searchButton" value="Search" /></td>
						<td>
							<svelte:component
								this={searchBar}
								bind:value={searchData.deviceId}
								id="filterDeviceId"
								className="searchInput"
								placeholder="Device ID"
							/>
						</td>
						<td>
							<svelte:component
								this={locationSelector}
								bind:value={searchData.locationId}
								id="filterLocation"
								className="searchInput"
								emptyValueLabel="-- Location --"
								disableEmptyValue={false}
							/>
						</td>
						{#each definitions.columnDefinitions as columnDefinition}
							{#if columnDefinition[0].showInMainPage}
								<td>
									<svelte:component
										this={searchBar}
										bind:value={searchData.columnData[columnDefinition[0].id].dataValue}
										id="filterColumn{columnDefinition[0].id}"
										className="searchInput"
										placeholder="{columnDefinition[0].name}"
									/>
								</td>
							{/if}
						{/each}
					</tr>
					{#each deviceResults as deviceResult}
						<tr>
							<td>
								<svelte:component
									this={checkoutButton}
									deviceId={deviceResult[0].deviceId}
									bind:currentLocationId={deviceResult[0].locationId}
									bind:currentLocationName={deviceResult[0].location}
								/>
							</td>
							<td>
								<svelte:component this={datetime} datetimeUtc={deviceResult[0].lastUpdated + 'Z'} />
							</td>
							<td class="centerContents monospace slightlyLargerFont">
								<a href="/edit/{deviceResult[0].deviceId}" class="block altLink">
									{deviceResult[0].deviceId}
								</a>
							</td>
							<td>{deviceResult[0].location}</td>
							{#each deviceResult[1] as columnValue, index}
								{#if definitions.columnDefinitions[index][0].showInMainPage}
									<td>{columnValue.dataValue}</td>
								{/if}
							{/each}
						</tr>
					{/each}
				</table>
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
	tr:not(.headerRow):hover {
		background-color: #dddddd;
	}

	#newDeviceLink {
		margin: 0.2em;
	}
	#searchButton {
		width: 100%;
	}
	:global(.searchInput) {
		box-sizing: border-box;
		width: 6em;
		min-width: 100%;
	}
</style>
