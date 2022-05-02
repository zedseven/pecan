<script lang="ts">
	// Imports
	import { appName } from '../constants';
	import { selectedLocation } from '../stores';
	import locationSelector from './locationSelector.svelte';
</script>

<header id="navBar">
	<div id="left">
		<a href="/"><h1 id="title">{appName}</h1></a>
	</div>
	<div id="right">
		<svelte:component this={locationSelector} bind:value={$selectedLocation} />
	</div>
</header>

<style>
	#navBar {
		padding: 10px;
		background-color: #222222;
	}
	#left {
		display: inline-block;
		width: 20%;
		vertical-align: middle;
	}
	#right {
		display: inline-block;
		width: 79%;
		vertical-align: middle;
	}
	#title {
		margin: 0;
	}
</style>
