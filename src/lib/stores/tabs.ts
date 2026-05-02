import { writable, derived } from 'svelte/store';
import { get, set as idbSet } from 'idb-keyval';

export type Tab = {
    id: string;
    name: string;
    input: string;
    imageSrc: string | null;
};

function createTabsStore() {
    const isBrowser = typeof window !== 'undefined';
    
    // We start with an empty array or a default tab
    const { subscribe, set, update } = writable<Tab[]>([]);

    if (isBrowser) {
        // Load asynchronously to not block main thread
        get('images64_tabs').then(stored => {
            if (stored && Array.isArray(stored) && stored.length > 0) {
                set(stored);
            }
        }).catch(err => console.error("Failed to load tabs from IDB", err));
    }

    // Debounce the persistence to IDB to avoid trashing disk/CPU
    let persistTimeout: any;
    function persist(tabs: Tab[]) {
        if (!isBrowser) return;
        clearTimeout(persistTimeout);
        persistTimeout = setTimeout(() => {
            idbSet('images64_tabs', tabs).catch(err => console.error("Failed to save tabs to IDB", err));
        }, 1000);
    }

    return {
        subscribe,
        set: (tabs: Tab[]) => {
            set(tabs);
            persist(tabs);
        },
        addTab: () => {
            const newTab: Tab = {
                id: crypto.randomUUID(),
                name: 'New Tab',
                input: '',
                imageSrc: null
            };
            update(tabs => {
                const next = [...tabs, newTab];
                persist(next);
                return next;
            });
            return newTab.id;
        },
        removeTab: (id: string) => update(tabs => {
            const next = tabs.filter(t => t.id !== id);
            persist(next);
            return next;
        }),
        updateTab: (id: string, data: Partial<Tab>) => update(tabs => {
            const next = tabs.map(t => t.id === id ? { ...t, ...data } : t);
            persist(next);
            return next;
        })
    };
}

export const tabs = createTabsStore();

// For activeTabId, we can keep using localStorage since it's tiny
let initialActiveId: string | null = null;
if (typeof window !== 'undefined') {
    initialActiveId = localStorage.getItem('images64_active_tab');
}

export const activeTabId = writable<string | null>(initialActiveId);
activeTabId.subscribe(id => {
    if (typeof window !== 'undefined' && id !== null) {
        localStorage.setItem('images64_active_tab', id);
    }
});

export const activeTab = derived(
    [tabs, activeTabId],
    ([$tabs, $activeTabId]) => $tabs.find(t => t.id === $activeTabId) || null
);
