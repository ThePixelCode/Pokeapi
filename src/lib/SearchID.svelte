<script lang="ts">
    export let callback: (id: number) => Promise<void>;

    let id: number = 0;
    let errors: string | undefined = undefined;

    async function submit() {
        if (id <= 0) {
            errors = "id should be more than 0";
            return;
        }
        errors = undefined;
        await callback(id);
    }
</script>

<form
    on:submit|preventDefault|stopPropagation={submit}
    class="m-2 p-3 w-1/3 bg-neutral-100 dark:bg-neutral-900"
>
    <div class="flex flex-row align-middle justify-items-center">
        <label for="id" class="basis-1/6 text-left">ID:</label>
        <input
            type="number"
            name="id"
            id="id"
            bind:value={id}
            class="text-black bg-neutral-400 rounded p-1 ml-1 basis-5/6"
        />
    </div>
    {#if errors !== undefined}
        <span class="text-red-700 text-sm my-2">{errors}</span>
    {/if}
    <button
        type="submit"
        class="my-2 p-1 bg-neutral-800 dark:bg-neutral-200 text-white dark:text-black w-full rounded-lg hover:bg-neutral-700 hover:dark:bg-neutral-300 no-underline hover:underline underline-offset-2 transition-colors ease-out"
        >Search</button
    >
</form>
