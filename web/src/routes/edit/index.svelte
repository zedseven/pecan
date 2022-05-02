<script lang="ts">
	import navBar from '../../components/navBar.svelte';
	import editDevice from '../../components/editDevice.svelte';
</script>

<svelte:head>
	<title>Add New Device</title>
</svelte:head>

<svelte:component this={navBar} />

<svelte:component this={editDevice} />
