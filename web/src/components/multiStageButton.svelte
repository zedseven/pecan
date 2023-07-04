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
	export let id = null;
	export let className = null;
	export let width = '9em';

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
	<button {id} class={className} disabled={true} style="width: {width};">{disabledText}</button>
{:else if clickState === ClickState.Loading}
	<button {id} class="{className} loading" style="width: {width};">{loadingText}</button>
{:else if clickState === ClickState.Primed}
	<button on:click={clicked} {id} class="{className} primed" style="width: {width};"
		>{primedText}</button
	>
{:else}
	<button on:click={clicked} {id} class={className} style="width: {width};">{defaultText}</button>
{/if}
