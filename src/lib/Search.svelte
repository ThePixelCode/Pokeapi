<script lang="ts">
    import { writable, type Writable } from "svelte/store";
    import SearchId from "./SearchID.svelte";
    import SearchName from "./SearchName.svelte";

    export let callback: (query: {
        id?: number;
        name?: string;
    }) => Promise<void>;
    export let searchFor: string;

    let searchId: Writable<boolean> = writable(true);

    async function toggle_search() {
        searchId.update((previous) => !previous);
    }

    async function set_search_to(id: boolean) {
        searchId.set(id);
    }
</script>

<div class="m-2 p-2 flex flex-row w-1/3 align-middle items-center">
    <h1 class="basis-3/4">Search {searchFor} For:</h1>
    <div class="basis-1/4 m-2 grid grid-flow-col">
        <button
            on:click|preventDefault|stopPropagation={() => set_search_to(true)}
            >ID</button
        >
        <button
            on:click|preventDefault|stopPropagation={toggle_search}
            class="mx-1 bg-white border-neutral-600 border-2 grid grid-flow-col grid-cols-2 align-middle items-center gap-1 rounded-xl w-[50px]"
        >
            <div
                class="aspect-square bg-black w-[13px] rounded transition-transform ease-in-out duration-200 col-start-2 {$searchId
                    ? '-translate-x-[18px]'
                    : ''}"
            ></div>
        </button>
        <button
            on:click|preventDefault|stopPropagation={() => set_search_to(false)}
            >Name</button
        >
    </div>
</div>

{#if $searchId}
    <SearchId
        callback={async (id) => {
            await callback({ id: id });
        }}
    />
{/if}
{#if !$searchId}
    <SearchName
        callback={async (name) => {
            await callback({ name: name });
        }}
    />
{/if}
