<script lang="ts">
    import type { PokemonMove } from "../types/pokemon";
    import { computePosition, flip, shift } from "@floating-ui/dom";

    export let pokemonMove: PokemonMove;

    let show: boolean = false;

    async function update() {
        let button = document.querySelector(
            `#button-${pokemonMove.move.name}`,
        ) as HTMLButtonElement;
        let tooltip = document.querySelector(
            `#tooltip-${pokemonMove.move.name}`,
        ) as HTMLDivElement;

        let { x, y } = await computePosition(button, tooltip, {
            placement: "top",
            middleware: [flip(), shift({ padding: 5 })],
        });

        Object.assign(tooltip.style, {
            left: `${x}px`,
            top: `${y}px`,
        });
    }

    async function toogleTooltip() {
        show = !show;

        if (show) {
            await update();
        }
    }

    async function hideTooltip() {
        show = false;
    }
</script>

<div>
    <h1>
        {pokemonMove.move.name}
        <button
            id="button-{pokemonMove.move.name}"
            aria-describedby="tooltip-{pokemonMove.move.name}"
            class="bg-neutral-200 dark:bg-neutral-900 aspect-square min-w-5"
            on:click|preventDefault|stopPropagation={toogleTooltip}>?</button
        >
    </h1>
</div>

<div
    id="tooltip-{pokemonMove.move.name}"
    class="absolute w-max p-3 max-h-[50vh] overflow-auto bg-neutral-200 dark:bg-black {show
        ? ''
        : 'hidden'}"
>
    <button
        class="absolute top-2 right-2 z-0 w-5"
        on:click|preventDefault|stopPropagation={hideTooltip}>X</button
    >
    {#each pokemonMove.version_group_details as detailsPerGameVersion}
        <div class="mr-5">
            On {detailsPerGameVersion.version_group.name},
            {pokemonMove.move.name} is obtained via {detailsPerGameVersion
                .move_learn_method.name} at level {detailsPerGameVersion.level_learned_at}
        </div>
    {/each}
</div>
