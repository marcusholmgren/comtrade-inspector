// app/vite.config.ts
// Configures Vite, Vitest, and SvelteKitPWA plugin for the application.
// This file exists to define the build options, test suites, and PWA setup.
// RELEVANT FILES: app/svelte.config.js, app/package.json, app/src/app.d.ts

import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';
import { execSync } from 'child_process';
import { readFileSync } from 'fs';
import { playwright } from '@vitest/browser-playwright';
import { SvelteKitPWA } from '@vite-pwa/sveltekit';

const getGitHash = () => {
	try {
		return execSync('git rev-parse --short HEAD').toString().trim();
	} catch {
		return 'unknown';
	}
};

const getVersion = () => {
	try {
		const pkg = JSON.parse(readFileSync('./package.json', 'utf-8'));
		return pkg.version;
	} catch {
		return '0.0.0';
	}
};

const base = process.env.BASE_URL || '';

export default defineConfig({
	define: {
		__APP_VERSION__: JSON.stringify(getVersion()),
		__GIT_HASH__: JSON.stringify(getGitHash())
	},
	plugins: [
		tailwindcss(),
		sveltekit(),
		SvelteKitPWA({
			registerType: 'autoUpdate',
			manifest: {
				name: 'COMTRADE inspector',
				short_name: 'COMTRADE inspector',
				description: 'Common Format for Transient Data Exchange (COMTRADE) data analysis tools',
				theme_color: '#111418',
				background_color: '#111418',
				display: 'standalone',
				start_url: base ? `${base}/` : '/',
				scope: base ? `${base}/` : '/',
				icons: [
					{
						src: base ? `${base}/favicon.svg` : '/favicon.svg',
						sizes: 'any',
						type: 'image/svg+xml',
						purpose: 'any'
					},
					{
						src: base ? `${base}/favicon.svg` : '/favicon.svg',
						sizes: 'any',
						type: 'image/svg+xml',
						purpose: 'maskable'
					}
				],
				screenshots: [
					{
						src: base ? `${base}/comtrade-inspector.jpg` : '/comtrade-inspector.jpg',
						sizes: '1080x720',
						type: 'image/jpeg',
						form_factor: 'wide',
						label: 'COMTRADE Inspector Dashboard'
					}
				]
			},
			workbox: {
				globPatterns: ['**/*.{js,css,html,ico,png,svg,wasm,webmanifest}'],
				additionalManifestEntries: [
					{
						url: base ? `${base}/index.html` : '/index.html',
						revision: getGitHash()
					}
				]
			}
		})
	],
	server: {
		fs: {
			allow: ['..']
		},
		headers: {
			'Cross-Origin-Embedder-Policy': 'require-corp',
			'Cross-Origin-Opener-Policy': 'same-origin'
		}
	},
	assetsInclude: ['**/*.wasm'],
	optimizeDeps: {
		exclude: ['comtrade_rust']
	},
	test: {
		expect: { requireAssertions: true },
		projects: [
			{
				extends: './vite.config.ts',
				test: {
					name: 'client',
					browser: {
						enabled: true,
						provider: playwright(),
						instances: [{ browser: 'chromium' }]
					},
					include: ['src/**/*.svelte.{test,spec}.{js,ts}'],
					exclude: ['src/lib/server/**'],
					setupFiles: ['./vitest-setup-client.ts']
				}
			},
			{
				extends: './vite.config.ts',
				test: {
					name: 'server',
					environment: 'node',
					include: ['src/**/*.{test,spec}.{js,ts}'],
					exclude: ['src/**/*.svelte.{test,spec}.{js,ts}']
				}
			}
		]
	}
});
