import { MODE } from '../types/Global';
import { writable } from 'svelte/store';

export const mode = writable(MODE.STACK);
