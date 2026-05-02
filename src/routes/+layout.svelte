<script lang="ts">
    import "../app.css";
    import { onMount } from "svelte";
    import Sidebar from "$lib/components/Sidebar.svelte";
    import ToastContainer from "$lib/components/ToastContainer.svelte";
    import { settings } from "$lib/stores/settings";

    onMount(() => {
        // Apply dark mode on mount
        settings.subscribe(($settings) => {
            if ($settings.darkMode) {
                document.documentElement.classList.add("dark");
            } else {
                document.documentElement.classList.remove("dark");
            }
        });
    });
</script>

<div class="flex h-screen w-full overflow-hidden font-sans transition-colors duration-200">
    <Sidebar />
    <main class="flex-1 h-full overflow-y-auto relative bg-[var(--bg-base)]">
        <slot />
    </main>
    <ToastContainer />
</div>
