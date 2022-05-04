<script lang="ts">
	// Component Data
	export let value = '';
	export let placeholder = 'Search';
</script>

<input type="search" {placeholder} bind:value />
