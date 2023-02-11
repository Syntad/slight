import type { Item } from '../types/StackOverflowTypes';
import { writable, type Writable } from 'svelte/store';

export const results: Writable<Item[]> = writable([]);
