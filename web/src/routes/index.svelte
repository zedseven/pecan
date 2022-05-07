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

{#await loadingPromise}
	<svelte:component this={loading} />
{:then loadingResult}
	{#if loadingResult.ok}
		<h3 id="newDeviceLink"><a href="/edit">Add a New Device</a></h3>
		<form on:submit|preventDefault={onSearch} method="post">
			<table>
				<tr class="headerRow">
					<th>Device ID</th>
					<th>Location</th>
					<th>Last Updated</th>
					{#each columnDefinitions as columnDefinition}
						<th>{columnDefinition.name}</th>
					{/each}
					<th colspan="2" />
				</tr>
				<tr class="headerRow">
					<td>
						<svelte:component
							this={searchBar}
							placeholder="Filter by Device ID"
							bind:value={searchData.deviceId}
						/>
					</td>
					<td>
						<svelte:component
							this={locationSelector}
							bind:value={searchData.locationId}
							emptyValueLabel="-- Filter by Location --"
							disableEmptyValue={false}
						/>
					</td>
					<td />
					{#each columnDefinitions as columnDefinition}
						<td>
							<svelte:component
								this={searchBar}
								placeholder="Filter by {columnDefinition.name}"
								bind:value={searchData.columnData[columnDefinition.id].dataValue}
							/>
						</td>
					{/each}
					<td colspan="2"><input type="submit" value="Search" /></td>
				</tr>
				{#each deviceResults as deviceResult}
					<tr>
						<td class="monospace">{deviceResult[0].deviceId}</td>
						<td>{deviceResult[0].location}</td>
						<td>
							<svelte:component this={datetime} datetimeUtc={deviceResult[0].lastUpdated + 'Z'} />
						</td>
						{#each deviceResult[1] as columnValue}
							<td>{columnValue.dataValue}</td>
						{/each}
						<td>
							<svelte:component
								this={checkoutButton}
								deviceId={deviceResult[0].deviceId}
								bind:currentLocationId={deviceResult[0].locationId}
								bind:currentLocationName={deviceResult[0].location}
							/>
						</td>
						<td><a href="/edit/{deviceResult[0].deviceId}">Edit</a></td>
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

<style>
	tr:not(.headerRow):hover {
		background-color: #dddddd;
	}

	#newDeviceLink {
		margin: 0.2em;
	}
</style>
