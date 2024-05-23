<script lang="ts">
    import { onDestroy, onMount } from "svelte";
    import { writable, type Unsubscriber } from "svelte/store";

    let dark = writable(false);

    let unsubscriber: Unsubscriber;

    onDestroy(() => {
        unsubscriber();
    });

    function sync_theme() {
        if (!("theme" in localStorage)) {
            dark.set(
                document.documentElement.classList.contains("dark")
                    ? true
                    : false,
            );
        } else {
            dark.set(localStorage.getItem("theme") == "light" ? false : true);
        }

        unsubscriber = dark.subscribe((dark_theme) => {
            if (dark_theme) {
                document.documentElement.classList.add("dark");
                localStorage.setItem("theme", "dark");
            } else {
                document.documentElement.classList.remove("dark");
                localStorage.setItem("theme", "light");
            }
        });
    }

    async function toggle_dark_theme() {
        dark.update((previous_theme) => {
            return !previous_theme;
        });
    }

    onMount(sync_theme);
</script>

<header class="p-2 flex flex-row text-3xl bg-neutral-900 text-white">
    <div class="grow">
        <span>Header</span>
    </div>
    <div class="w-[200px] px-2 grid grid-flow-col align-middle items-center">
        <button
            on:click|preventDefault={() => {
                dark.set(true);
            }}>Dark</button
        >
        <button
            class="w-[40px] h-[20px] bg-white border-neutral-600 border-2 grid grid-flow-col grid-cols-2 align-middle items-center gap-1 rounded-xl p-[2px]"
            on:click|preventDefault={toggle_dark_theme}
        >
            <div
                class="aspect-square bg-blue-400 w-[13px] rounded transition-transform ease-in-out duration-200 col-start-2 dark:-translate-x-[17px]"
            ></div>
        </button>
        <button
            on:click|preventDefault={() => {
                dark.set(false);
            }}>Light</button
        >
    </div>
</header>
