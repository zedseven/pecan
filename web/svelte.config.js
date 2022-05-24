// Imports
import adapter from '@sveltejs/adapter-static';
import preprocess from 'svelte-preprocess';

// Constants
const buildMode = process.env.BUILD_MODE || 'development';
const projectVersion = process.env.PROJECT_VERSION || 'unknown';

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
				__PROJECT_VERSION__: projectVersion,
			},
			mode: buildMode, // https://github.com/sveltejs/kit/issues/1258#issuecomment-874482104
		},
	},
	preprocess: preprocess(),
};

export default config;
