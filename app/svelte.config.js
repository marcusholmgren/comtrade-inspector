// app/svelte.config.js
// Configures the SvelteKit project options including adapter-static and PWA registration settings.
// This file exists to set up compile options, preprocessors, and routing base paths.
// RELEVANT FILES: app/vite.config.ts, app/package.json

import adapter from '@sveltejs/adapter-static';
import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';

/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://svelte.dev/docs/kit/integrations
	// for more information about preprocessors
	preprocess: vitePreprocess(),

	kit: {
		// adapter-auto only supports some environments, see https://svelte.dev/docs/kit/adapter-auto for a list.
		// If your environment is not supported, or you settled on a specific environment, switch out the adapter.
		// See https://svelte.dev/docs/kit/adapters for more information about adapters.
		adapter: adapter({
			pages: 'build',
			assets: 'build',
			fallback: 'index.html',
			precompress: false
		}),
		paths: {
			base: process.env.BASE_URL && process.env.BASE_URL !== '/' ? process.env.BASE_URL : ''
		},
		serviceWorker: {
			register: false
		}
	}
};

export default config;
