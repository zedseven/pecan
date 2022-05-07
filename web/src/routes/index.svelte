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
	import { handleNetworkResponse, Ok, postData, sanitiseObjectMapToArray } from '../util';

	// Component Data
	let columnDefinitions;
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

		if (data != null) {
			inputData.deviceId = data.deviceId;
			inputData.locationId = data.locationId ? data.locationId : null;
			inputData.columnData = sanitiseObjectMapToArray(data.columnData);
		}

		const searchUrl = '/api/devices/search';
		return postData(searchUrl, inputData).then(async (searchResult) => {
			// If there was an error, return it for processing below
			if (!searchResult.ok) return searchResult;

			// Store the results
			columnDefinitions = searchResult.value.columnDefinitions;
			deviceResults = searchResult.value.deviceResults;

			return searchResult;
		});
	};

	const onSearch = async (event) => {
		event.preventDefault();

		loadingPromise = sendSearch(searchData);
	};

	// Load the devices
	let loadingPromise = sendSearch(null).then(async (queryResult) => {
		// If there was an error, return it for processing below
		if (!queryResult.ok) return queryResult;

		// Set up the searchData for binding
		for (const columnDefinition of columnDefinitions) {
			searchData.columnData[columnDefinition.id] = {
				columnDefinitionId: columnDefinition.id,
				dataValue: null,
			};
		}

		return Ok({});
	});
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
						<th colspan="2">Last Updated</th>
						<th><label for="filterDeviceId" class="block">Device ID</label></th>
						<th><label for="filterLocation" class="block">Location</label></th>
						{#each columnDefinitions as columnDefinition}
							<th>
								<label for="filterColumn{columnDefinition.id}" class="block">
									{columnDefinition.name}
								</label>
							</th>
						{/each}
					</tr>
					<tr class="headerRow">
						<td colspan="2"><input type="submit" id="searchButton" value="Search" /></td>
						<td>
							<svelte:component
								this={searchBar}
								bind:value={searchData.deviceId}
								id="filterDeviceId"
								className="maxWidth"
								placeholder="Filter by Device ID"
							/>
						</td>
						<td>
							<svelte:component
								this={locationSelector}
								bind:value={searchData.locationId}
								id="filterLocation"
								className="maxWidth"
								emptyValueLabel="-- Filter by Location --"
								disableEmptyValue={false}
							/>
						</td>
						{#each columnDefinitions as columnDefinition}
							<td>
								<svelte:component
									this={searchBar}
									bind:value={searchData.columnData[columnDefinition.id].dataValue}
									id="filterColumn{columnDefinition.id}"
									className="maxWidth"
									placeholder="Filter by {columnDefinition.name}"
								/>
							</td>
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
							{#each deviceResult[1] as columnValue}
								<td>{columnValue.dataValue}</td>
							{/each}
						</tr>
					{/each}
				</table>
			</form>
		{:else}
			<svelte:component this={responseError} error={loadingResult.error} />
		{/if}
	{:catch}
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
</style>
