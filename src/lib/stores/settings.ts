import { writable } from 'svelte/store';

export type Settings = {
    darkMode: boolean;
    defaultSavePath: string | null;
};

// We will load this from local storage or Tauri store later
const defaultSettings: Settings = {
    darkMode: true,
    defaultSavePath: null
};

let initial = defaultSettings;
const isBrowser = typeof window !== 'undefined';
if (isBrowser) {
    const stored = localStorage.getItem('images64_settings');
    if (stored) {
        try { initial = { ...defaultSettings, ...JSON.parse(stored) }; } catch (e) {}
    }
}

export const settings = writable<Settings>(initial);

settings.subscribe(val => {
    if (isBrowser) {
        localStorage.setItem('images64_settings', JSON.stringify(val));
    }
});
