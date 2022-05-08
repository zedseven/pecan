import adapter from '@sveltejs/adapter-static';
import preprocess from 'svelte-preprocess';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://github.com/sveltejs/svelte-preprocess
	// for more information about preprocessors
	preprocess: preprocess(),

	kit: {
		adapter: adapter(),
		prerender: {
			default: true,
		},
		vite: {
			mode: process.env.BUILD_MODE || 'development', // https://github.com/sveltejs/kit/issues/1258#issuecomment-874482104
			build: {
				minify: process.env.BUILD_MODE || 'development' !== 'development',
			},
		},
	},
};

export default config;
