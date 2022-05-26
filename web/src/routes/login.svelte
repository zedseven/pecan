<script lang="ts">
	// Imports
	import navBar from '../components/navBar.svelte';
	import loading from '../components/loading.svelte';
	import responseError from '../components/responseError.svelte';
	import couldntConnect from '../components/couldntConnect.svelte';
	import { appNameCased } from '../constants';
	import { Ok, postData } from '../util';

	// Component Data
	let username = '';
	let password = '';
	let loadingPromise = Promise.resolve(Ok({}));
	let errorMessage = null;

	const onSubmit = async (event) => {
		event.preventDefault();
		let inputData = {
			username: username,
			password: password,
		};

		// Reset page data
		errorMessage = null;
		password = '';

		// Push it to the server
		const loginUrl = '/api/authenticate';
		loadingPromise = postData(loginUrl, inputData).then((loginResult) => {
			console.log(loginResult);

			// Redirect to the index page if successful
			if (loginResult.ok) {
				window.location = '/';
			} else {
				errorMessage = loginResult.error.message;
			}

			return Ok({});
		});
	};
</script>

<svelte:head>
	<title>Log In - {appNameCased}</title>
</svelte:head>

<svelte:component this={navBar} />

<div id="content">
	{#await loadingPromise}
		<svelte:component this={loading} />
	{:then loadingResult}
		{#if loadingResult.ok}
			{#if errorMessage}
				<p>{errorMessage}</p>
			{/if}
			<form on:submit|preventDefault={onSubmit} method="post">
				<table>
					<tr class="noHoverDarken">
						<td><label for="username" class="block">User: </label></td>
						<td><input bind:value={username} id="username" type="text" required={true} /></td>
					</tr>
					<tr class="noHoverDarken">
						<td><label for="password" class="block">Password: </label></td>
						<td><input bind:value={password} id="password" type="password" required={true} /></td>
					</tr>
				</table>
				<input type="submit" value="Log In" />
			</form>
		{:else}
			<svelte:component this={responseError} error={loadingResult.error} />
		{/if}
	{:catch}
		{@debug loadingPromise}
		<svelte:component this={couldntConnect} />
	{/await}
</div>

<style lang="scss">
</style>
