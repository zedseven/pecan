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
	import { appName } from '../constants';
	import { fetchDefinitions } from '../stores';
	import { Ok, postData, redirectIfNotLoggedIn, sanitiseObjectMapToArray } from '../util';

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
		return postData(searchUrl, inputData).then(async (searchResult) =>
			processSearchResult(searchResult, true),
		);
	};
	const defaultSearch = async () => {
		const searchUrl = '/api/devices/search/default';
		return postData(searchUrl, null).then(processSearchResult);
	};

	const processSearchResult = async (searchResult, fillHoles = false) => {
		// If there was an error, return it for processing below
		if (!searchResult.ok) return searchResult;

		// Store the results
		deviceResults = searchResult.value.deviceResults;

		// Fill in holes if requested
		if (fillHoles) fillResultHoles();

		return searchResult;
	};
	// Fill in holes where there aren't device data entries (for columns that were added after the device was created)
	const fillResultHoles = () => {
		for (let deviceResult of deviceResults) {
			// The lengths match, there's no problem
			if (deviceResult[1].length == definitions.columnDefinitions.length) continue;
			// Otherwise, compare the entries to find holes
			for (let index = 0; index < deviceResult[1].length; index++) {
				// Compare the column definition IDs
				if (deviceResult[1][index].columnDefinitionId == definitions.columnDefinitions[index][0].id)
					continue;

				// Fill the hole
				deviceResult[1].splice(index, 0, {
					id: null,
					deviceKeyInfoId: deviceResult[1][index].deviceKeyInfoId,
					columnDefinitionId: definitions.columnDefinitions[index][0].id,
					dataValue: '',
				});
				index++;
			}
		}
	};

	const onSearch = async (event) => {
		event.preventDefault();

		loadingPromise = sendSearch(searchData);
	};

	// Load the devices
	let loadingPromise = Promise.all([
		redirectIfNotLoggedIn(),
		fetchDefinitions(),
		defaultSearch(),
	]).then((combinedResult) => {
		let definitionsResult = combinedResult[1];
		let loadResult = combinedResult[2];

		// If there was an error with either query, return it for processing below
		if (!definitionsResult.ok) return definitionsResult;
		if (!loadResult.ok) return loadResult;

		// Store the definitions
		definitions = definitionsResult.value;

		// Set up the searchData for binding
		for (const columnDefinition of definitions.columnDefinitions) {
			searchData.columnData[columnDefinition[0].id] = {
				columnDefinitionId: columnDefinition[0].id,
				dataValue: columnDefinition[0].defaultValue,
			};
		}

		fillResultHoles();

		return Ok({});
	});
</script>

<svelte:head>
	<title>{appName}</title>
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
					<tr class="noHoverDarken">
						<th><label for="filterDeviceId" class="block">Device ID</label></th>
						{#each definitions.columnDefinitions as columnDefinition}
							{#if columnDefinition[0].showInMainPage}
								<th>
									<label for="filterColumn{columnDefinition[0].id}" class="block">
										{columnDefinition[0].name}
									</label>
								</th>
							{/if}
						{/each}
						<th><label htmlFor="filterLocation" className="block">Location</label></th>
						<th colSpan="2">Last-Updated</th>
					</tr>
					<tr class="noHoverDarken">
						<td>
							<svelte:component
								this={searchBar}
								bind:value={searchData.deviceId}
								id="filterDeviceId"
								className="searchInput"
								placeholder="Device ID"
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
										placeholder={columnDefinition[0].name}
									/>
								</td>
							{/if}
						{/each}
						<td>
							<svelte:component
								this={locationSelector}
								bind:value={searchData.locationId}
								id="filterLocation"
								className="searchInput"
								disableEmptyValue={false}
							/>
						</td>
						<td colSpan="2"><input type="submit" id="searchButton" value="Search" /></td>
					</tr>
					{#each deviceResults as deviceResult}
						<tr>
							<td class="centerContents monospace slightlyLargerFont">
								<a href="/edit/{deviceResult[0].deviceId}" class="block altLink">
									{deviceResult[0].deviceId}
								</a>
							</td>
							{#each deviceResult[1] as columnValue, index}
								{#if definitions.columnDefinitions[index][0].showInMainPage}
									<td>{columnValue.dataValue}</td>
								{/if}
							{/each}
							<td>{deviceResult[0].location}</td>
							<td class="centerContents">
								<svelte:component this={datetime} datetimeUtc={deviceResult[0].lastUpdated + 'Z'} />
							</td>
							<td>
								<svelte:component
									this={checkoutButton}
									deviceId={deviceResult[0].deviceId}
									bind:currentLocationId={deviceResult[0].locationId}
									bind:currentLocationName={deviceResult[0].location}
								/>
							</td>
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

<style lang="scss">
	td {
		padding: 1px 4px;
	}

	#newDeviceLink {
		margin: 0.2em;
	}
	#searchButton {
		width: 100%;
	}
	:global(.searchInput) {
		width: 5em;
		min-width: 100%;
	}
</style>
