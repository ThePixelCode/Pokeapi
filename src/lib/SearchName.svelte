<script lang="ts">
    export let callback: (name: string) => Promise<void>;

    let name: string = "";
    let errors: string | undefined = undefined;

    async function submit() {
        if (name.trim().length === 0) {
            errors = "name cannot be empty";
            return;
        }
        errors = undefined;
        await callback(name.trim());
    }
</script>

<form
    on:submit|preventDefault|stopPropagation={submit}
    class="m-2 p-3 w-1/3 bg-neutral-100 dark:bg-neutral-900"
>
    <div class="flex flex-row align-middle justify-items-center">
        <label for="name" class="basis-1/6 text-left">Name:</label>
        <input
            type="text"
            name="name"
            id="name"
            bind:value={name}
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
