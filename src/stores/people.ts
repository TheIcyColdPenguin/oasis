import { writable } from 'svelte/store';
import type { Person } from '../types';

export const people = writable<Person[]>([]);
