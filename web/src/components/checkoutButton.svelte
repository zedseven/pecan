<script lang="ts">
	// Imports
	import multiStageButton from './multiStageButton.svelte';
	import { selectedLocation } from '../stores';
	import { Ok, postData } from '../util';

	// Component Data
	export let deviceId;
	export let currentLocationId;
	export let currentLocationName;

	// The function that runs when the button is clicked
	const clicked = function () {
		let inputData = {
			deviceId,
			locationId: $selectedLocation,
		};

		const checkoutUrl = '/api/devices/checkout';
		postData(checkoutUrl, inputData).then(async (checkoutResult) => {
			// If there was an error, return it for processing below
			if (!checkoutResult.ok) return checkoutResult;

			// Set the state
			currentLocationId = checkoutResult.value.locationId;
			currentLocationName = checkoutResult.value.locationName;

			return Ok({});
		});
	};
</script>

<svelte:component
	this={multiStageButton}
	clickedFunction={clicked}
	defaultText="Assign to me"
	disabledText="Already assigned"
	isDisabled={$selectedLocation === currentLocationId || $selectedLocation == null}
/>
