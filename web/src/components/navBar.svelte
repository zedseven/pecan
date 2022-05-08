<script lang="ts">
	// Imports
	import { appName } from '../constants';
	import { selectedLocation } from '../stores';
	import globalStyle from './globalStyle.svelte';
	import locationSelector from './locationSelector.svelte';

	// Set the logo filetype based on SVG support
	let svgFiletype = document.implementation.hasFeature("http://www.w3.org/TR/SVG11/feature#Image", "1.1") ? 'svg' : 'png';
</script>

<svelte:head>
	<link rel="icon" href="/favicon.png" type="image/png" />
	<link rel="icon" href="/favicon.svg" type="image/svg" />
</svelte:head>

<header id="navBar" class="noSelect">
	<div id="primary">
		<a href="/" id="titleLink"><img src="/logo.{svgFiletype}" alt="logo" id="logo"><h1 id="title">{appName}</h1></a>
	</div>
	<div id="secondary">
		<label for="locationSelector">Location: </label>
		<svelte:component
			this={locationSelector}
			bind:value={$selectedLocation}
			id="locationSelector"
			emptyValueLabel="-- Your Location --"
		/>
	</div>
</header>

<svelte:component this={globalStyle} />

<style>
	#navBar {
		padding: 10px;
		color: #ffffff;
		background-color: #222222;
	}
	#primary {
		display: inline-block;
		width: 20%;
		vertical-align: middle;
	}
	#secondary {
		display: inline-block;
		vertical-align: middle;
	}
	#titleLink {
		display: block;
	}
	#logo {
		display: inline-block;
		margin-right: 0.5em;
		width: 2.3em;
		vertical-align: middle;
	}
	#title {
		display: inline-block;
		vertical-align: middle;
		margin: 0;
		text-transform: lowercase;
	}
</style>
