<script lang="ts">
	// Imports
	import { appNameLowercase, buildVersion, buildDate } from '../constants';
	import globalStyle from './globalStyle.svelte';

	// Set the logo filetype based on SVG support
	let logoFileName = document.implementation.hasFeature(
		'http://www.w3.org/TR/SVG11/feature#Image',
		'1.1',
	)
		? 'favicon.svg' // SVGs scale infinitely so it's not a separate file
		: 'favicon-large.png';
</script>

<svelte:head>
	<link rel="icon" href="/favicon.png" type="image/png" />
	<link rel="icon" href="/favicon.svg" type="image/svg" />
</svelte:head>

<header id="navBar" class="noSelect">
	<div id="primary">
		<!-- prettier-ignore -->
		<a href="/" id="titleLink">
			<img src="/{logoFileName}" alt="logo" id="logo" /><h1 id="title">{appNameLowercase}</h1>
		</a>
	</div>
	<div id="secondary">
		<!--<label for="locationSelector">Location: </label>
		<svelte:component
			this={locationSelector}
			bind:value={$selectedLocation}
			id="locationSelector"
			emptyValueLabel="-- Your Location --"
		/>-->
		<p class="headerInfo" title={buildDate}>{buildVersion}</p>
	</div>
</header>

<svelte:component this={globalStyle} />

<style lang="scss">
	#navBar {
		position: fixed;
		top: 0;
		left: 0;
		right: 0;
		z-index: 5;
		box-sizing: border-box;
		height: 56px;
		padding: 10px;
		color: #ffffff;
		background-color: #222222;
	}
	:global(#content) {
		margin-top: 56px; /* Matching the height of #navBar */
	}
	#primary {
		display: inline-block;
		min-width: 15em;
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
		margin: -3px 1em 0 0;
		text-transform: lowercase;
	}
	.headerInfo {
		display: inline-block;
		margin: 0 2em;
	}

	@media print {
		:global {
			#navBar {
				display: none;
			}
			#content {
				margin-top: 0;
			}
		}
	}
</style>
