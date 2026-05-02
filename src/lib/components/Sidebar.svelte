<script lang="ts">
    import { Plus, X, Image as ImageIcon, Settings as SettingsIcon, Moon, Sun, Edit2 } from "lucide-svelte";
    import { tabs, activeTabId } from "$lib/stores/tabs";
    import { settings } from "$lib/stores/settings";
    import SettingsModal from "./SettingsModal.svelte";

    let showSettings = false;
    let editingTabId: string | null = null;
    let editingName = "";

    $: currentIndex = $tabs.findIndex(t => t.id === $activeTabId) + 1;
    $: totalTabs = $tabs.length;

    function addNewTab() {
        const id = tabs.addTab();
        $activeTabId = id;
    }

    function closeTab(id: string, event: MouseEvent) {
        event.stopPropagation();
        tabs.removeTab(id);
        if ($activeTabId === id) {
            $activeTabId = $tabs.length > 0 ? $tabs[$tabs.length - 1].id : null;
        }
    }

    function toggleDarkMode() {
        $settings.darkMode = !$settings.darkMode;
    }

    function startRenaming(id: string, currentName: string, event: MouseEvent) {
        event.stopPropagation();
        editingTabId = id;
        editingName = currentName;
    }

    function saveRename() {
        if (editingTabId && editingName.trim()) {
            tabs.updateTab(editingTabId, { name: editingName.trim() });
        }
        editingTabId = null;
    }

    function handleRenameKey(e: KeyboardEvent) {
        if (e.key === 'Enter') saveRename();
        if (e.key === 'Escape') editingTabId = null;
    }
</script>

<aside class="w-64 border-r border-[var(--border-soft)] bg-[var(--bg-surface)] flex flex-col h-full shrink-0 transition-colors duration-200">
    <div class="p-4 border-b border-[var(--border-soft)] flex items-center justify-between">
        <div class="flex items-center gap-2">
            <img src="/logos/icon.png" alt="Images64 Logo" class="w-8 h-8 object-contain drop-shadow-sm select-none" draggable="false" />
            <div class="flex flex-col gap-0.5">
                <span class="font-bold text-lg leading-none text-[var(--text-main)]">Images64</span>
                {#if totalTabs > 0}
                    <div class="inline-flex px-1.5 py-0.5 bg-blue-100/80 dark:bg-blue-900/30 text-blue-700 dark:text-blue-400 rounded text-[11px] font-bold tracking-wider uppercase items-center justify-center w-max shadow-sm border border-blue-200/50 dark:border-blue-800/30">Tab {currentIndex}/{totalTabs}</div>
                {/if}
            </div>
        </div>
        <button on:click={addNewTab} class="p-1.5 hover:bg-[var(--bg-surface-hover)] rounded-md transition-colors text-[var(--text-muted)] hover:text-[var(--text-main)]" title="New Tab">
            <Plus class="w-4 h-4" />
        </button>
    </div>

    <div class="flex-1 overflow-y-auto p-2 space-y-1">
        {#each $tabs as tab (tab.id)}
            <button 
                class="w-full text-left px-3 py-2 rounded-md flex items-center justify-between group transition-colors duration-150 border { $activeTabId === tab.id ? 'bg-[var(--bg-base)] text-blue-600 dark:text-blue-400 font-semibold shadow-sm border-[var(--border-soft)]' : 'border-transparent text-[var(--text-muted)] hover:bg-[var(--bg-surface-hover)] hover:text-[var(--text-main)]' }"
                on:click={() => $activeTabId = tab.id}
                on:dblclick={(e) => startRenaming(tab.id, tab.name, e)}
            >
                {#if editingTabId === tab.id}
                    <!-- svelte-ignore a11y-autofocus -->
                    <input 
                        type="text" 
                        bind:value={editingName} 
                        on:blur={saveRename}
                        on:keydown={handleRenameKey}
                        on:click|stopPropagation
                        autofocus
                        class="w-full bg-[var(--bg-surface)] px-1 py-0.5 rounded border border-blue-500 text-sm focus:outline-none text-[var(--text-main)]"
                    />
                {:else}
                    <span class="truncate pr-2 text-sm">{tab.name}</span>
                    <div class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-all">
                        <div 
                            class="p-1 rounded hover:bg-[var(--border-soft)] text-[var(--text-muted)] hover:text-[var(--text-main)]"
                            on:click={(e) => startRenaming(tab.id, tab.name, e)}
                            title="Rename"
                            role="button"
                            tabindex="0"
                            on:keydown={(e) => e.key === 'Enter' && startRenaming(tab.id, tab.name, e as any)}
                        >
                            <Edit2 class="w-3 h-3" />
                        </div>
                        <div 
                            class="p-1 rounded hover:bg-red-100 dark:hover:bg-red-900/30 text-red-400 hover:text-red-600 dark:hover:text-red-400"
                            on:click={(e) => closeTab(tab.id, e)}
                            title="Close"
                            role="button"
                            tabindex="0"
                            on:keydown={(e) => e.key === 'Enter' && closeTab(tab.id, e as any)}
                        >
                            <X class="w-3 h-3" />
                        </div>
                    </div>
                {/if}
            </button>
        {/each}
        {#if $tabs.length === 0}
            <div class="text-center text-sm text-[var(--text-muted)] py-8">
                No active tabs.<br>Click + to create one.
            </div>
        {/if}
    </div>

    <div class="p-4 border-t border-[var(--border-soft)] flex items-center justify-between text-[var(--text-muted)]">
        <button on:click={() => showSettings = true} class="p-2 hover:bg-[var(--bg-surface-hover)] hover:text-[var(--text-main)] rounded-md transition-colors flex items-center gap-2 text-sm" title="Settings">
            <SettingsIcon class="w-4 h-4" />
            <span>Settings</span>
        </button>
        <button on:click={toggleDarkMode} class="p-2 hover:bg-[var(--bg-surface-hover)] hover:text-[var(--text-main)] rounded-md transition-colors" title="Toggle Theme">
            {#if $settings.darkMode}
                <Sun class="w-4 h-4" />
            {:else}
                <Moon class="w-4 h-4" />
            {/if}
        </button>
    </div>
</aside>

{#if showSettings}
    <SettingsModal on:close={() => showSettings = false} />
{/if}
