<script lang="ts">
	import navBar from '../../components/navBar.svelte';
	import editDevice from '../../components/editDevice.svelte';
	import { page } from '$app/stores';

	let device = $page.params['device'];
</script>

<svelte:head>
	<title>Editing Device {device}</title>
</svelte:head>

<svelte:component this={navBar} />

<svelte:component this={editDevice} deviceId={device} />
