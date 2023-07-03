<script lang="ts">
	// Type Definitions
	enum ClickState {
		NotClicked,
		Primed,
		Loading,
	}

	// Component Data
	export let clickedFunction;
	export let defaultText;
	export let primedText = 'Are you sure?';
	export let loadingText = 'Loading...';
	export let disabledText = 'Disabled';
	export let isDisabled = false;
	export let primeTimeout = 5000; // 5 seconds

	let clickState = ClickState.NotClicked;

	// The function that runs when the button is clicked
	const clicked = function (event) {
		event.preventDefault();

		if (isDisabled) return;

		if (clickState == ClickState.Primed) {
			clickState = ClickState.Loading;

			clickedFunction();

			clickState = ClickState.NotClicked;
		} else if (clickState === ClickState.NotClicked) {
			clickState = ClickState.Primed;

			// Reset the state if not clicked again within the time limit
			setTimeout(function () {
				if (clickState === ClickState.Primed) {
					clickState = ClickState.NotClicked;
				}
			}, primeTimeout);
		}
	};
</script>

{#if isDisabled}
	<button disabled={true}>{disabledText}</button>
{:else if clickState === ClickState.Loading}
	<button class="loading">{loadingText}</button>
{:else if clickState === ClickState.Primed}
	<button on:click={clicked} class="primed">{primedText}</button>
{:else}
	<button on:click={clicked}>{defaultText}</button>
{/if}

<style lang="scss">
	button {
		width: 9em;
	}
</style>
