import { MODE } from '../types/globalTypes';
import { writable } from 'svelte/store';

export const mode = writable(MODE.STACK);
