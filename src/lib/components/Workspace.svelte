<script lang="ts">
    import { type Tab, tabs } from "$lib/stores/tabs";
    import { toasts } from "$lib/stores/toasts";
    import { UploadCloud, Image as ImageIcon, Download, Maximize2, Copy, Trash2, MoreVertical, Play, Activity, ChevronDown, ChevronUp } from "lucide-svelte";
    import { save, open } from "@tauri-apps/plugin-dialog";
    import { writeFile } from "@tauri-apps/plugin-fs";
    import { writeText } from "@tauri-apps/plugin-clipboard-manager";
    import { invoke } from "@tauri-apps/api/core";
    import { getCurrentWebview } from "@tauri-apps/api/webview";
    import { settings } from "$lib/stores/settings";
    import { get } from "svelte/store";
    import FullscreenGallery from "./FullscreenGallery.svelte";
    import { onMount, onDestroy } from "svelte";

    export let tab: Tab;

    let inputAreaElement: HTMLTextAreaElement;
    let isDragging = false;
    let isProcessing = false;
    let showOptions = false;
    let showFullscreen = false;
    let isFitToView = false;

    // Computed properties
    $: hasImage = !!tab.imageSrc;
    // Auto collapse input area logic
    $: showInput = !hasImage;

    const HUGE_STRING_LIMIT = 100000; // ~100KB

    // Chunking logic for massive strings using Rust
    let loadedChunks = 0;
    let displayChunks = "";
    let isChunking = false;

    // Analytics formatting
    $: formatSize = (bytes: number) => {
        if (bytes < 1024) return bytes + " B";
        if (bytes < 1024 * 1024) return (bytes / 1024).toFixed(2) + " KB";
        return (bytes / 1024 / 1024).toFixed(2) + " MB";
    };
    
    $: inputSize = tab.input.length;
    // A base64 string length is roughly 4/3 of the actual binary size
    $: estBinarySize = inputSize > 0 ? (inputSize * 3) / 4 : 0;
    
    // Guess Mime from data string
    $: mimeType = tab.imageSrc 
        ? (tab.imageSrc.match(/data:(.*?);/)?.[1] || "Unknown") 
        : (tab.input.startsWith("iVBORw") ? "image/png" : 
           tab.input.startsWith("/9j/") ? "image/jpeg" : 
           tab.input.startsWith("R0lGOD") ? "image/gif" : 
           tab.input.startsWith("UklGR") ? "image/webp" : "Unknown");

    // Native Drag Drop Listeners
    let unlistenDrop: () => void;
    
    onMount(async () => {
        unlistenDrop = await getCurrentWebview().onDragDropEvent((event) => {
            if (event.payload.type === 'over') {
                isDragging = true;
            } else if (event.payload.type === 'drop') {
                isDragging = false;
                if (event.payload.paths && event.payload.paths.length > 0) {
                    processFileFromPath(event.payload.paths[0]);
                }
            } else {
                isDragging = false; // cancel or leave
            }
        });
    });

    onDestroy(() => {
        if (unlistenDrop) unlistenDrop();
    });

    // Watch for tab.input changes to reset chunking state
    $: {
        if (tab.input.length > HUGE_STRING_LIMIT) {
            if (loadedChunks === 0) {
                displayChunks = ""; // Reset
            }
        } else {
            loadedChunks = 0;
            displayChunks = tab.input;
        }
    }

    $: displayValue = (tab.input.length > HUGE_STRING_LIMIT)
        ? (loadedChunks === 0 
            ? `[Large Base64 String Loaded - ${(tab.input.length / 1024 / 1024).toFixed(2)} MB]\n\n(Click anywhere here to load the first chunk of text)`
            : displayChunks + `\n\n...[Loaded ${loadedChunks} chunk(s). Click to load more]`)
        : displayChunks;

    async function loadNextChunk() {
        if (tab.input.length <= HUGE_STRING_LIMIT || isChunking) return;
        
        // If we've already loaded the whole thing, stop.
        if (loadedChunks * HUGE_STRING_LIMIT >= tab.input.length) return;

        isChunking = true;
        try {
            // Fulfill the requirement to handle the chunk slicing in Rust
            const nextChunk: string = await invoke("get_string_chunk", { 
                text: tab.input, 
                chunkSize: HUGE_STRING_LIMIT, 
                chunkIndex: loadedChunks 
            });
            displayChunks += nextChunk;
            loadedChunks++;
        } catch (err) {
            console.error("Chunking error:", err);
        } finally {
            isChunking = false;
        }
    }

    // Instead of reactive updates on every keystroke, we sync on blur or manually
    function handleBlur() {
        if (inputAreaElement && tab.input.length <= HUGE_STRING_LIMIT) {
            tabs.updateTab(tab.id, { input: inputAreaElement.value });
        }
    }

    function handlePaste(e: ClipboardEvent) {
        const pastedText = e.clipboardData?.getData('text');
        if (pastedText && pastedText.length > HUGE_STRING_LIMIT) {
            e.preventDefault(); // Stop the browser from injecting massive text into DOM and hanging
            tabs.updateTab(tab.id, { input: pastedText });
            loadedChunks = 0;
            displayChunks = "";
            toasts.add({ message: `Pasted massive string (${(pastedText.length / 1024 / 1024).toFixed(2)} MB)`, type: "info" });
        }
    }

    async function handleConvertClick() {
        let valToConvert = tab.input;
        if (tab.input.length <= HUGE_STRING_LIMIT && inputAreaElement) {
            valToConvert = inputAreaElement.value;
            tabs.updateTab(tab.id, { input: valToConvert });
        }
        
        if (!valToConvert.trim()) return;
        await processBase64(valToConvert);
    }

    async function processBase64(rawStr: string) {
        if (!rawStr) {
            tabs.updateTab(tab.id, { imageSrc: null, name: 'New Tab' });
            return;
        }

        isProcessing = true;

        try {
            // Call heavy lifting Rust function
            const base64Data: string = await invoke("process_base64_string", { input: rawStr });
            
            // Test if valid
            const img = new Image();
            img.onload = () => {
                tabs.updateTab(tab.id, { 
                    imageSrc: base64Data,
                    name: `Image (${img.width}x${img.height})`
                });
                toasts.add({ message: "Image decoded successfully", type: "success" });
                isProcessing = false;
            };
            img.onerror = () => {
                toasts.add({ message: "Invalid Base64 string", type: "error" });
                tabs.updateTab(tab.id, { imageSrc: null, name: 'Invalid Base64' });
                isProcessing = false;
            };
            img.src = base64Data;

        } catch (err) {
            toasts.add({ message: "Error processing base64", type: "error" });
            isProcessing = false;
        }
    }

    async function handleFilePicker() {
        try {
            const filePath = await open({
                multiple: false,
                title: "Select File"
            });
            if (filePath && typeof filePath === 'string') {
                await processFileFromPath(filePath);
            }
        } catch (err) {
            toasts.add({ message: "Failed to open file", type: "error" });
        }
    }

    async function processFileFromPath(path: string) {
        isProcessing = true;
        try {
            const base64Data: string = await invoke("read_file_to_base64", { path });
            // Update input string with raw data without the data:image/... prefix for user editability
            let rawStr = base64Data;
            if (base64Data.includes("base64,")) {
                rawStr = base64Data.split("base64,")[1];
            }
            tabs.updateTab(tab.id, { input: rawStr });
            toasts.add({ message: "File loaded into input. Ready to convert.", type: "info" });
        } catch(err) {
            toasts.add({ message: String(err), type: "error" });
        } finally {
            isProcessing = false;
        }
    }

    // Web-based drop still needs to read file since we can't easily get path from DragEvent without native plugins
    function handleDrop(e: DragEvent) {
        e.preventDefault();
        isDragging = false;
        
        if (e.dataTransfer?.files.length) {
            const file = e.dataTransfer.files[0];
            
            if (file.type.startsWith('image/')) {
                // Reverse Conversion: Image to Base64
                const reader = new FileReader();
                reader.onload = (ev) => {
                    const base64Str = ev.target?.result as string;
                    const rawStr = base64Str.split("base64,")[1] || base64Str;
                    tabs.updateTab(tab.id, { input: rawStr });
                    toasts.add({ message: "Image loaded into input. Ready to convert.", type: "info" });
                };
                reader.readAsDataURL(file);
            } else {
                // Text file drop
                const reader = new FileReader();
                reader.onload = (ev) => {
                    const result = ev.target?.result as string;
                    tabs.updateTab(tab.id, { input: result });
                    toasts.add({ message: "File loaded into input. Ready to convert.", type: "info" });
                };
                reader.readAsText(file);
            }
        }
    }

    function handleDragOver(e: DragEvent) {
        e.preventDefault();
        isDragging = true;
    }

    function handleDragLeave(e: DragEvent) {
        e.preventDefault();
        isDragging = false;
    }

    function clear() {
        tabs.updateTab(tab.id, { input: '', imageSrc: null, name: 'New Tab' });
        loadedChunks = 0;
        displayChunks = "";
        showOptions = false;
    }

    // Output Actions
    async function saveImage() {
        showOptions = false;
        if (!tab.imageSrc) return;

        try {
            const mimeMatch = tab.imageSrc.match(/data:(.*?);/);
            const ext = mimeMatch ? mimeMatch[1].split('/')[1] : 'png';
            const $settings = get(settings);

            let filePath;
            
            if ($settings.defaultSavePath) {
                const fileName = `image_${Date.now()}.${ext}`;
                const sep = $settings.defaultSavePath.includes('\\') ? '\\' : '/';
                filePath = `${$settings.defaultSavePath}${sep}${fileName}`;
            } else {
                filePath = await save({
                    filters: [{ name: 'Image', extensions: [ext, 'jpg', 'jpeg', 'png', 'gif', 'webp', 'bmp', 'svg', 'ico', 'avif', 'tiff'] }],
                    defaultPath: `image.${ext}`
                });
            }

            if (filePath) {
                // Save entirely via Rust to bypass strict JS capabilities and memory limits
                await invoke("save_base64_to_file", { path: filePath, base64Data: tab.imageSrc });
                toasts.add({ message: `Image saved to ${filePath}`, type: "success" });
            }
        } catch (err) {
            console.error("Save image error:", err);
            toasts.add({ message: `Failed to save image: ${err}`, type: "error" });
        }
    }

    async function copyBase64() {
        showOptions = false;
        try {
            // Copy entirely via Rust to avoid IPC/JS size limits on standard plugin manager
            await invoke("copy_to_clipboard", { text: tab.input });
            toasts.add({ message: "Base64 copied to clipboard", type: "success" });
        } catch (err) {
            console.error("Copy base64 error:", err);
            toasts.add({ message: `Failed to copy: ${err}`, type: "error" });
        }
    }

    // Click outside handler for dropdown
    function windowClick() {
        if (showOptions) showOptions = false;
    }
</script>

<svelte:window on:click={windowClick} />

<div 
    class="flex-1 flex flex-col h-full relative bg-[var(--bg-base)]"
    role="region"
    aria-label="Workspace"
>
    {#if isDragging}
        <div class="absolute inset-0 z-50 bg-blue-500/20 dark:bg-blue-500/10 border-4 border-dashed border-blue-500 rounded-lg flex items-center justify-center backdrop-blur-sm pointer-events-none transition-all">
            <div class="bg-[var(--bg-surface)] p-6 rounded-xl shadow-xl flex flex-col items-center gap-4 border border-[var(--border-soft)]">
                <UploadCloud class="w-12 h-12 text-blue-500" />
                <p class="text-lg font-bold text-[var(--text-main)]">Drop text file or image here</p>
            </div>
        </div>
    {/if}

    <!-- Header Section (Input) -->
    <div class="p-6 shrink-0 transition-all duration-300 {isFitToView ? 'hidden' : (showInput ? 'flex-1 flex flex-col justify-center' : 'border-b border-[var(--border-soft)] bg-[var(--bg-surface)]')}">
        <div class="max-w-4xl mx-auto w-full flex flex-col h-full gap-3">
            <div class="flex items-center justify-between">
                <div class="flex items-center gap-4">
                    <h2 class="font-semibold flex items-center gap-2 text-[var(--text-main)]">
                        <ImageIcon class="w-4 h-4 text-[var(--text-muted)]" />
                        Base64 Input
                    </h2>
                    {#if inputSize > 0}
                        <div class="hidden sm:flex items-center gap-3 px-3 py-1 bg-[var(--bg-base)] rounded-full text-[11px] font-medium text-[var(--text-main)] border border-[var(--border-soft)] shadow-sm">
                            <span class="flex items-center gap-1" title="Base64 Text Size"><Activity class="w-3 h-3 text-blue-500"/> {formatSize(inputSize)}</span>
                            <span class="w-1 h-1 rounded-full bg-[var(--border-soft)]"></span>
                            <span title="Estimated File Binary Size">Est. {formatSize(estBinarySize)}</span>
                            {#if mimeType !== 'Unknown'}
                                <span class="w-1 h-1 rounded-full bg-[var(--border-soft)]"></span>
                                <span class="text-blue-600 dark:text-blue-400 font-semibold">{mimeType}</span>
                            {/if}
                        </div>
                    {/if}
                </div>
                <div class="flex items-center gap-2">
                    {#if !showInput && hasImage}
                        <button on:click={clear} class="text-xs text-red-500 hover:text-red-600 flex items-center gap-1 font-medium transition-colors bg-[var(--bg-surface)] px-2 py-1 rounded shadow-sm border border-[var(--border-soft)] hover:bg-[var(--bg-base)]">
                            <Trash2 class="w-3 h-3" /> Clear
                        </button>
                    {/if}
                </div>
            </div>

            <textarea 
                bind:this={inputAreaElement}
                value={displayValue}
                on:blur={handleBlur}
                on:paste={handlePaste}
                on:click={loadNextChunk}
                readonly={tab.input.length > HUGE_STRING_LIMIT}
                placeholder="Paste your Base64 string here..."
                class="w-full flex-1 {showInput ? 'min-h-[200px]' : 'h-16'} p-3 rounded-lg border border-[var(--border-soft)] bg-[var(--bg-base)] shadow-inner focus:ring-2 focus:ring-blue-500 focus:outline-none resize-none font-mono text-xs transition-all select-text text-[var(--text-main)] {tab.input.length > HUGE_STRING_LIMIT ? 'cursor-pointer' : ''}"
            ></textarea>
            
            <div class="flex items-center justify-between mt-1">
                <button 
                    on:click={handleFilePicker}
                    class="px-3 py-1.5 bg-[var(--bg-base)] hover:bg-[var(--bg-surface-hover)] border border-[var(--border-soft)] rounded-md transition-colors font-medium text-[var(--text-main)] flex items-center gap-2 text-sm shadow-sm"
                >
                    <UploadCloud class="w-4 h-4" />
                    Load from File
                </button>

                <button 
                    on:click={handleConvertClick}
                    disabled={isProcessing}
                    class="px-4 py-1.5 bg-blue-600 hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed text-white rounded-md flex items-center gap-2 transition-colors font-medium text-sm shadow-sm"
                >
                    {#if isProcessing}
                        <div class="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin"></div>
                        Processing...
                    {:else}
                        <Play class="w-4 h-4" /> Convert
                    {/if}
                </button>
            </div>
        </div>
    </div>

    <!-- Separator / Fit Toggle Button -->
    {#if hasImage}
        <div class="relative h-0 w-full flex justify-center z-20">
            <button 
                on:click={() => isFitToView = !isFitToView}
                class="absolute {isFitToView ? 'top-3' : '-top-3.5'} bg-[var(--bg-surface)] hover:bg-[var(--bg-surface-hover)] border border-[var(--border-soft)] rounded-full px-4 py-1 shadow-sm text-[10px] font-bold tracking-wider uppercase text-[var(--text-muted)] hover:text-[var(--text-main)] transition-all flex items-center gap-1"
                title={isFitToView ? "Restore View" : "Fit Image to View"}
            >
                {#if isFitToView}
                    <ChevronDown class="w-3.5 h-3.5" /> Restore View
                {:else}
                    <ChevronUp class="w-3.5 h-3.5" /> Fit to View
                {/if}
            </button>
        </div>
    {/if}

    <!-- Output Section -->
    {#if hasImage}
        <div class="flex-1 flex flex-col bg-[var(--bg-base)] overflow-hidden relative">
            
            <!-- Clean Toolbar -->
            <div class="h-12 border-b border-[var(--border-soft)] bg-[var(--bg-surface)] flex items-center justify-end px-4 gap-2 shrink-0">
                <button on:click={() => showFullscreen = true} class="flex items-center gap-1.5 px-3 py-1.5 hover:bg-[var(--bg-surface-hover)] rounded-md transition-colors text-sm font-medium text-[var(--text-main)]">
                    <Maximize2 class="w-4 h-4" /> Fullscreen
                </button>

                <!-- 3 Dots Menu -->
                <div class="relative">
                    <button 
                        on:click|stopPropagation={() => showOptions = !showOptions} 
                        class="p-1.5 hover:bg-[var(--bg-surface-hover)] rounded-md transition-colors"
                    >
                        <MoreVertical class="w-5 h-5 text-[var(--text-muted)] hover:text-[var(--text-main)]" />
                    </button>

                    {#if showOptions}
                        <div class="absolute right-0 top-full mt-1 w-40 bg-[var(--bg-surface)] border border-[var(--border-soft)] rounded-md shadow-lg z-20 py-1" on:click|stopPropagation>
                            <button on:click={copyBase64} class="w-full text-left px-4 py-2 hover:bg-[var(--bg-surface-hover)] flex items-center gap-2 text-sm text-[var(--text-main)]">
                                <Copy class="w-4 h-4" /> Copy Base64
                            </button>
                            <button on:click={saveImage} class="w-full text-left px-4 py-2 hover:bg-[var(--bg-surface-hover)] flex items-center gap-2 text-sm text-blue-600 dark:text-blue-400 font-medium">
                                <Download class="w-4 h-4" /> Save Image
                            </button>
                        </div>
                    {/if}
                </div>
            </div>

            <!-- Dedicated Image Area -->
            <div class="flex-1 {isFitToView ? 'overflow-hidden p-4' : 'overflow-auto p-6'} relative bg-[var(--bg-base)]">
                <div class="{isFitToView ? 'w-full h-full' : 'min-h-full min-w-max'} flex">
                    <div class="m-auto relative shadow-md border border-[var(--border-soft)] rounded-md overflow-hidden bg-[url('data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyMCIgaGVpZ2h0PSIyMCI+CjxyZWN0IHdpZHRoPSIyMCIgaGVpZ2h0PSIyMCIgZmlsbD0iI2ZmZiIgLz4KPHJlY3Qgd2lkdGg9IjEwIiBoZWlnaHQ9IjEwIiBmaWxsPSIjZjBmMGZmIiAvPgo8cmVjdCB4PSIxMCIgeT0iMTAiIHdpZHRoPSIxMCIgaGVpZ2h0PSIxMCIgZmlsbD0iI2YwZjBmZiIgLz4KPC9zdmc+')] dark:bg-[url('data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIyMCIgaGVpZ2h0PSIyMCI+CjxyZWN0IHdpZHRoPSIyMCIgaGVpZ2h0PSIyMCIgZmlsbD0iIzIyMiIgLz4KPHJlY3Qgd2lkdGg9IjEwIiBoZWlnaHQ9IjEwIiBmaWxsPSIjMzMzIiAvPgo8cmVjdCB4PSIxMCIgeT0iMTAiIHdpZHRoPSIxMCIgaGVpZ2h0PSIxMCIgZmlsbD0iIzMzMyIgLz4KPC9zdmc+')] {isFitToView ? 'flex items-center justify-center max-w-full max-h-full' : ''}">
                        <img 
                            src={tab.imageSrc} 
                            alt="Decoded output" 
                            class="block {isFitToView ? 'max-w-full max-h-full object-contain' : 'max-w-none'}"
                            draggable="false"
                        />
                    </div>
                </div>
            </div>
        </div>
    {/if}

    {#if showFullscreen}
        <FullscreenGallery on:close={() => showFullscreen = false} />
    {/if}
</div>