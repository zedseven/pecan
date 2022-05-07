<script lang="ts">
	// Imports
	import { appName } from '../constants';
	import { selectedLocation } from '../stores';
	import locationSelector from './locationSelector.svelte';
</script>

<header id="navBar" class="noSelect">
	<div id="primary">
		<a href="/"><h1 id="title">{appName}</h1></a>
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
	#title {
		margin: 0;
	}
</style>
