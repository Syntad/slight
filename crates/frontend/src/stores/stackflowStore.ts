import type { Item } from '../types/stackflowTypes';
import { writable, type Writable } from 'svelte/store';

export const results: Writable<Item[]> = writable([]);
