<script lang="ts">
    import type { PokemonSprite } from "../types/pokemon";

    export let pokemonSprite: PokemonSprite;
    export let pokemonName: string;
    let imageClass: string = "aspect-square w-32";
</script>

<div
    class="grid {pokemonSprite.front_female === null
        ? 'grid-cols-2'
        : 'grid-cols-4'}"
>
    <img
        class={imageClass}
        src={pokemonSprite.front_default}
        alt="{pokemonName} front default"
    />
    <img
        class={imageClass}
        src={pokemonSprite.front_shiny}
        alt="{pokemonName} front shiny"
    />
    {#if pokemonSprite.front_female !== null}
        <img
            class={imageClass}
            src={pokemonSprite.front_default}
            alt="{pokemonName} front default"
        />
        <img
            class={imageClass}
            src={pokemonSprite.front_shiny}
            alt="{pokemonName} front shiny"
        />
    {/if}
    <img
        class={imageClass}
        src={pokemonSprite.back_default}
        alt="{pokemonName} back default"
    />
    <img
        class={imageClass}
        src={pokemonSprite.back_shiny}
        alt="{pokemonName} back shiny"
    />
    {#if pokemonSprite.front_female !== null}
        <img
            class={imageClass}
            src={pokemonSprite.back_default}
            alt="{pokemonName} back default"
        />
        <img
            class={imageClass}
            src={pokemonSprite.back_shiny}
            alt="{pokemonName} back shiny"
        />
    {/if}
</div>
