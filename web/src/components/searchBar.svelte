<script lang="ts">
	// Component Data
	export let value = '';
	export let id = null;
	export let className = null;
	export let placeholder = 'Search';
</script>

<input type="search" {id} class={className} {placeholder} bind:value />
