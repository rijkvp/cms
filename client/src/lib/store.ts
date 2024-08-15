import type { Section } from '$lib/models/Section';
import { writable, type Writable } from 'svelte/store';

export const sections: Writable<Array<Section>> = writable([]);

