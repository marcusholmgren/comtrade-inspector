// app/src/lib/store.ts
// This file contains the Svelte store for the application.
// This file exists to share data between components.
// RELEVANT FILES: app/src/routes/+page.svelte, app/src/routes/info/+page.svelte

import { writable } from 'svelte/store';

export const analysisResult = writable<unknown>(null);
