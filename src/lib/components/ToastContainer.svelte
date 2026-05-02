<script lang="ts">
    import { toasts } from "$lib/stores/toasts";
    import { X, CheckCircle, AlertCircle, Info } from "lucide-svelte";
    import { fade, slide } from "svelte/transition";
</script>

<div class="fixed bottom-4 right-4 z-50 flex flex-col gap-2 pointer-events-none">
    {#each $toasts as toast (toast.id)}
        <div 
            transition:slide
            class="pointer-events-auto flex items-center gap-3 px-4 py-3 rounded-lg shadow-lg border min-w-[250px] max-w-sm
            {toast.type === 'success' ? 'bg-green-50 border-green-200 text-green-800 dark:bg-green-950/50 dark:border-green-900 dark:text-green-300' : ''}
            {toast.type === 'error' ? 'bg-red-50 border-red-200 text-red-800 dark:bg-red-950/50 dark:border-red-900 dark:text-red-300' : ''}
            {toast.type === 'info' ? 'bg-blue-50 border-blue-200 text-blue-800 dark:bg-blue-950/50 dark:border-blue-900 dark:text-blue-300' : ''}"
        >
            <div class="shrink-0">
                {#if toast.type === 'success'}
                    <CheckCircle class="w-5 h-5" />
                {:else if toast.type === 'error'}
                    <AlertCircle class="w-5 h-5" />
                {:else}
                    <Info class="w-5 h-5" />
                {/if}
            </div>
            <p class="text-sm font-medium flex-1">{toast.message}</p>
            <button 
                on:click={() => toasts.remove(toast.id)}
                class="shrink-0 p-1 rounded-md hover:bg-black/5 dark:hover:bg-white/10 transition-colors"
            >
                <X class="w-4 h-4" />
            </button>
        </div>
    {/each}
</div>
