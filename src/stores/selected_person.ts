import { invoke } from '@tauri-apps/api';
import { writable } from 'svelte/store';

import { currentNotes } from './currentnotes';

import type { Note, Person } from '../types';

export const selectedPerson = writable<Person | null>(null);

selectedPerson.subscribe(async person => {
    if (person) {
        const notes = await invoke<Note[]>('get_notes', { personId: person.id });
        currentNotes.set(notes);
    }
});
