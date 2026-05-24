// app/src/app.d.ts
// Declares global type definitions and references for Vite PWA virtual modules.
// This file exists to enable TypeScript support for window globals and PWA virtual modules.
// RELEVANT FILES: app/vite.config.ts, app/src/routes/+layout.svelte

/// <reference types="vite-plugin-pwa/info" />
/// <reference types="vite-plugin-pwa/client" />

// See https://svelte.dev/docs/kit/types#app.d.ts
// for information about these interfaces
declare global {
	const __APP_VERSION__: string;
	const __GIT_HASH__: string;

	namespace App {
		// interface Error {}
		// interface Locals {}
		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
	}
}

export {};
