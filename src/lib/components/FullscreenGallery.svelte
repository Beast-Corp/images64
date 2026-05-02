<script lang="ts">
    import { tabs, activeTabId } from "$lib/stores/tabs";
    import { X, ChevronLeft, ChevronRight } from "lucide-svelte";
    import { createEventDispatcher, onMount } from "svelte";

    const dispatch = createEventDispatcher();
    
    $: imageTabs = $tabs.filter(t => t.imageSrc);
    
    // Internal state for navigation
    let currentGalleryId = $activeTabId;
    $: currentIndex = imageTabs.findIndex(t => t.id === currentGalleryId);

    onMount(() => {
        if (currentIndex === -1 && imageTabs.length > 0) {
            currentGalleryId = imageTabs[0].id;
        }
    });

    function next(e?: MouseEvent) {
        if (e) e.stopPropagation();
        if (imageTabs.length === 0) return;
        let newIdx = currentIndex + 1;
        if (newIdx >= imageTabs.length) newIdx = 0;
        currentGalleryId = imageTabs[newIdx].id;
    }

    function prev(e?: MouseEvent) {
        if (e) e.stopPropagation();
        if (imageTabs.length === 0) return;
        let newIdx = currentIndex - 1;
        if (newIdx < 0) newIdx = imageTabs.length - 1;
        currentGalleryId = imageTabs[newIdx].id;
    }

    function handleKeydown(e: KeyboardEvent) {
        if (e.key === 'ArrowRight') next();
        if (e.key === 'ArrowLeft') prev();
        if (e.key === 'Escape') close();
    }

    function close(e?: MouseEvent) {
        if (e) e.stopPropagation();
        // User requested: "pressing X should return me to the tab where I originally was at."
        // We simply close the gallery and leave $activeTabId untouched.
        dispatch('close');
    }

</script>

<svelte:window on:keydown={handleKeydown} />

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div class="fixed inset-0 z-[100] bg-black/95 flex flex-col backdrop-blur-md" on:click={close}>
    
    <!-- Top Bar -->
    <div class="flex items-center justify-between p-4 text-white z-10 shrink-0" on:click|stopPropagation>
        <div class="font-medium bg-black/50 px-3 py-1.5 rounded-full text-sm">
            {currentIndex + 1} / {imageTabs.length} - {imageTabs[currentIndex]?.name || 'Unknown'}
        </div>
        <button on:click={close} class="p-2 hover:bg-white/10 rounded-full transition-colors bg-black/50" title="Close (Esc)">
            <X class="w-6 h-6" />
        </button>
    </div>

    <!-- Image Area -->
    <div class="flex-1 relative flex items-center justify-center overflow-hidden" on:click|stopPropagation>
        {#if imageTabs.length > 1}
            <button on:click={prev} class="absolute left-4 p-3 bg-black/50 hover:bg-black/80 text-white rounded-full transition-colors z-20 shadow-lg">
                <ChevronLeft class="w-8 h-8" />
            </button>
        {/if}

        {#if imageTabs[currentIndex]}
            <img 
                src={imageTabs[currentIndex].imageSrc} 
                alt="Fullscreen" 
                class="max-w-full max-h-full object-contain pointer-events-none select-none drop-shadow-2xl"
                draggable="false"
            />
        {/if}

        {#if imageTabs.length > 1}
            <button on:click={next} class="absolute right-4 p-3 bg-black/50 hover:bg-black/80 text-white rounded-full transition-colors z-20 shadow-lg">
                <ChevronRight class="w-8 h-8" />
            </button>
        {/if}
    </div>
</div>