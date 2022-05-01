<script lang="ts">
	import { page } from '$app/stores';
	import { Err, Ok } from '/src/util';

	const url = '/api/devices/columns';
	let resultData = fetch(url, { method: 'get' }).then(async (response) => {
		if (!response.ok) {
			return Err({
				status: response.status,
				statusText: response.statusText,
				message: await response.text(),
			});
		}
		return Ok(await response.json());
	});
</script>

<p>Hi! Edit here. {$page.params['device']}</p>
