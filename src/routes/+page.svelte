<script lang="ts">
    import { onMount } from "svelte";
    import { tabs, activeTabId, activeTab } from "$lib/stores/tabs";
    import Workspace from "$lib/components/Workspace.svelte";
    import { Image as ImageIcon } from "lucide-svelte";

    onMount(() => {
        // Initialize with one tab if empty
        if ($tabs.length === 0) {
            $activeTabId = tabs.addTab();
        }
    });
</script>

<div class="h-full w-full flex flex-col">
    {#if $activeTab}
        {#key $activeTab.id}
            <Workspace tab={$activeTab} />
        {/key}
    {:else}
        <div class="flex-1 flex flex-col items-center justify-center text-gray-400 dark:text-gray-600">
            <ImageIcon class="w-16 h-16 mb-4 opacity-50" />
            <p class="text-lg">No active session</p>
            <button 
                on:click={() => $activeTabId = tabs.addTab()}
                class="mt-4 px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-md transition-colors text-sm font-medium"
            >
                Create New Tab
            </button>
        </div>
    {/if}
</div>
