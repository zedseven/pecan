<script lang="ts">
	import editDevice from '../../components/editDevice.svelte';
	import { page } from '$app/stores';
</script>

<svelte:component this={editDevice} deviceId={$page.params['device']} />
