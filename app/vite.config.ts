import tailwindcss from '@tailwindcss/vite';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import { execSync } from 'child_process';
import { readFileSync } from 'fs';
import { playwright } from '@vitest/browser-playwright';

const getGitHash = () => {
	try {
		return execSync('git rev-parse --short HEAD').toString().trim();
	} catch (e) {
		return 'unknown';
	}
};

const getVersion = () => {
	try {
		const pkg = JSON.parse(readFileSync('./package.json', 'utf-8'));
		return pkg.version;
	} catch (e) {
		return '0.0.0';
	}
};

export default defineConfig({
	define: {
		__APP_VERSION__: JSON.stringify(getVersion()),
		__GIT_HASH__: JSON.stringify(getGitHash())
	},
	plugins: [tailwindcss(), sveltekit()],
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
