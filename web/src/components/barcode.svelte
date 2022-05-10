<script lang="ts">
	// Component Data
	export let data;
	export let id = 'barcode'; // Must be unique because it's fetched by JsBarcode
	export let displayValue = false;
	export let height = 25;
	let libLoaded = false;

	// Render the barcode
	const render = () => {
		if (libLoaded) JsBarcode('#' + id, data, { displayValue, height, margin: 0 });
	};

	// Library loaded
	const jsBarcodeLoaded = () => {
		libLoaded = true;
		render();
	};

	// On change, re-render
	$: {
		render();
	}
</script>

<svelte:head>
	<script src="/lib/JsBarcode.code128.min.js" on:load={jsBarcodeLoaded}></script>
</svelte:head>

<svg {id} />

<style lang="scss">
	svg {
		position: relative;
		z-index: -1;
	}
</style>
