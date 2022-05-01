// Disables all server-side rendering in development mode.
// Even if disabled for production, SvelteKit still attempts to render the
// server-side in development. This can cause problems if there are browser-only
// features or runtime-only URLs in use.
// https://github.com/sveltejs/kit/tree/master/packages/adapter-static#spa-mode
/** @type {import('@sveltejs/kit').Handle} */
export async function handle({ event, resolve }) {
	return await resolve(event, { ssr: false });
}
