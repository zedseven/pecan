<script lang="ts">
	import navBar from '../../components/navBar.svelte';
	import editDevice from '../../components/editDevice.svelte';
	import { appName } from '../../constants';
</script>

<svelte:head>
	<title>Add New Device - {appName}</title>
</svelte:head>

<svelte:component this={navBar} />

<svelte:component this={editDevice} />
