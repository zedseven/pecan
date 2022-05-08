import adapter from '@sveltejs/adapter-static';
import preprocess from 'svelte-preprocess';

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
				minify: process.env.BUILD_MODE || 'development' !== 'development',
				sourcemap: true,
			},
			mode: process.env.BUILD_MODE || 'development', // https://github.com/sveltejs/kit/issues/1258#issuecomment-874482104
		},
	},
	preprocess: preprocess(),
};

export default config;
