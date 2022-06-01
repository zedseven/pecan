// Imports
import adapter from '@sveltejs/adapter-static';
import preprocess from 'svelte-preprocess';

// Constants
const buildMode = process.env.BUILD_MODE || 'development';
const buildVersion = process.env.BUILD_VERSION || 'unknown';
const buildDate = process.env.BUILD_DATE || 'unknown';

// Exports
/** @type {import('@sveltejs/kit').Config} */
const config = {
	compilerOptions: { enableSourcemap: true },
	kit: {
		adapter: adapter(),
		prerender: {
			default: true,
			enabled: true,
		},
		vite: {
			build: {
				minify: buildMode !== 'development',
				sourcemap: true,
			},
			define: {
				__BUILD_VERSION__: buildVersion,
				__BUILD_DATE__: buildDate,
			},
			mode: buildMode, // https://github.com/sveltejs/kit/issues/1258#issuecomment-874482104
		},
	},
	preprocess: preprocess(),
};

export default config;
