<script lang="ts">
	// Imports
	import { selectedLocation } from '../stores';
	import { handleNetworkResponse, Ok, postData } from '../util';

	// Type Definitions
	enum ClickState {
		NotClicked,
		Primed,
		Loading,
	}

	// Component Data
	export let deviceId;
	export let currentLocationId;
	export let currentLocationName;
	let clickState = ClickState.NotClicked;

	// The function that runs when the button is clicked
	const clicked = function (event) {
		event.preventDefault();

		if (currentLocationId === $selectedLocation) return;

		if (clickState == ClickState.Primed) {
			clickState = ClickState.Loading;

			let inputData = {
				deviceId,
				locationId: $selectedLocation,
			};
			if (inputData.locationId == null) {
				clickState = ClickState.NotClicked;
				return;
			}

			const checkoutUrl = '/api/devices/checkout';
			postData(checkoutUrl, inputData)
				.then(handleNetworkResponse)
				.then(async (checkoutResult) => {
					// If there was an error, return it for processing below
					if (!checkoutResult.ok) return checkoutResult;

					// Set the state
					clickState = ClickState.NotClicked;
					currentLocationId = checkoutResult.value.locationId;
					currentLocationName = checkoutResult.value.locationName;

					return Ok({});
				});
		} else if (clickState === ClickState.NotClicked) {
			clickState = ClickState.Primed;
			// Reset the state if not clicked again in 5 seconds
			setTimeout(function () {
				if (clickState === ClickState.Primed) {
					clickState = ClickState.NotClicked;
				}
			}, 5000);
		}
	};
</script>

{#if $selectedLocation === null}
	<button disabled="disabled">Select your location</button>
{:else if $selectedLocation === currentLocationId}
	<button disabled="disabled">Already assigned</button>
{:else if clickState === ClickState.Loading}
	<button class="loading">Loading...</button>
{:else if clickState === ClickState.Primed}
	<button on:click={clicked} class="primed">Are you sure?</button>
{:else}
	<button on:click={clicked}>Assign to me</button>
{/if}
