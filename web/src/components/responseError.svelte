<script lang="ts">
	export let error;
</script>

<p>
	Received an error from the server: {error.status}
	{#if error.message}
		{error.statusText}: {error.message}
	{:else}
		{error.statusText}
	{/if}
</p>
