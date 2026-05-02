<script lang="ts">
    import { settings } from "$lib/stores/settings";
    import { open } from "@tauri-apps/plugin-dialog";
    import { X, FolderOpen } from "lucide-svelte";
    import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher();

    async function selectDefaultPath() {
        try {
            const selectedPath = await open({
                directory: true,
                multiple: false,
                title: "Select Default Save Directory"
            });
            if (selectedPath) {
                $settings.defaultSavePath = selectedPath;
            }
        } catch (err) {
            console.error(err);
        }
    }

    function clearDefaultPath() {
        $settings.defaultSavePath = null;
    }

</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div class="fixed inset-0 z-[200] bg-black/50 flex items-center justify-center backdrop-blur-sm" on:click={() => dispatch('close')}>
    <div 
        class="bg-white dark:bg-gray-900 rounded-xl shadow-2xl border border-gray-200 dark:border-gray-800 w-full max-w-md overflow-hidden"
        on:click|stopPropagation
    >
        <div class="flex items-center justify-between p-4 border-b border-gray-200 dark:border-gray-800">
            <h2 class="text-lg font-bold">Settings</h2>
            <button on:click={() => dispatch('close')} class="p-1 hover:bg-gray-100 dark:hover:bg-gray-800 rounded-md">
                <X class="w-5 h-5" />
            </button>
        </div>
        
        <div class="p-6 space-y-6 text-sm">
            <div class="space-y-2">
                <label class="font-medium flex items-center gap-2">Default Save Directory</label>
                <div class="flex items-center gap-2">
                    <input 
                        type="text" 
                        readonly 
                        value={$settings.defaultSavePath || 'Always ask for path'}
                        class="flex-1 px-3 py-2 bg-gray-100 dark:bg-gray-800 border border-gray-300 dark:border-gray-700 rounded-md focus:outline-none text-gray-600 dark:text-gray-400"
                    />
                    <button 
                        on:click={selectDefaultPath}
                        class="px-3 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-md flex items-center gap-2 transition-colors font-medium shrink-0"
                    >
                        <FolderOpen class="w-4 h-4" /> Browse
                    </button>
                </div>
                {#if $settings.defaultSavePath}
                    <button on:click={clearDefaultPath} class="text-red-500 hover:text-red-600 font-medium text-xs">Clear default path</button>
                {/if}
            </div>

            <div class="space-y-2">
                <label class="font-medium flex items-center justify-between cursor-pointer">
                    <span>Dark Mode</span>
                    <input 
                        type="checkbox" 
                        bind:checked={$settings.darkMode} 
                        class="sr-only peer"
                    >
                    <div class="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-blue-300 dark:peer-focus:ring-blue-800 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-blue-600 relative"></div>
                </label>
            </div>
        </div>
    </div>
</div>
