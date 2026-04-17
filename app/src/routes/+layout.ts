// app/src/routes/+layout.ts
// This file exports the layout configuration variables.
// This file exists because prerender and ssr variables should be in a separate .ts/.js file in SvelteKit.
// RELEVANT FILES: app/src/routes/+layout.svelte

export const prerender = true;
export const ssr = false;
