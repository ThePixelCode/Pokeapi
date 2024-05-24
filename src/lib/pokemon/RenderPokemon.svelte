<script lang="ts">
  import type { Pokemon } from "../types/pokemon";
  import { capitalize_first } from "../utils";
  import RenderPokemonMove from "./RenderPokemonMove.svelte";
  import RenderPokemonType from "./RenderPokemonType.svelte";
  import RenderPokemonStat from "./RenderPokemonStat.svelte";
  import RenderPokemonAbility from "./RenderPokemonAbility.svelte";
  import RenderPokemonSprites from "./RenderPokemonSprites.svelte";

  export let pokemon: Pokemon;
</script>

<div class="m-2 p-2">
  <h1>{`#${pokemon.id}`} Name: {capitalize_first(pokemon.name)}</h1>
  <RenderPokemonSprites
    pokemonSprite={pokemon.sprites}
    pokemonName={capitalize_first(pokemon.name)}
  />
  <div class="grid grid-cols-3">
    <div class="grid grid-cols-subgrid col-span-3">
      <div>Species</div>
      <div class="col-span-2">{capitalize_first(pokemon.species.name)}</div>
    </div>
    <div class="grid grid-cols-subgrid col-span-3">
      <div>Base Experience</div>
      <div class="col-span-2">{pokemon.base_experience}</div>
    </div>
    <div class="grid grid-cols-subgrid col-span-3">
      <div>Height</div>
      <div class="col-span-2">{pokemon.height * 10}cm</div>
    </div>
    <div class="grid grid-cols-subgrid col-span-3">
      <div>Weight</div>
      <div class="col-span-2">{pokemon.weight / 10}kg</div>
    </div>
    <div class="grid grid-cols-subgrid col-span-3">
      <div>Moves</div>
      <div class="max-h-[25vh] overflow-auto col-span-2">
        {#each pokemon.moves as move}
          <RenderPokemonMove pokemonMove={move} />
        {/each}
      </div>
    </div>
    <div class="grid grid-cols-subgrid col-span-3">
      <div>Types</div>
      <div class="max-h-[25vh] overflow-auto col-span-2">
        {#each pokemon.types as pokemonType}
          <RenderPokemonType {pokemonType} />
        {/each}
      </div>
    </div>
    <div class="grid grid-cols-subgrid col-span-3">
      <div>Stats</div>
      <div class="max-h-[25vh] overflow-auto col-span-2">
        {#each pokemon.stats as pokemonStat}
          <RenderPokemonStat {pokemonStat} />
        {/each}
      </div>
    </div>
    <div class="grid grid-cols-subgrid col-span-3">
      <div>Abilities</div>
      <div class="max-h-[25vh] overflow-auto col-span-2">
        {#each pokemon.abilities as pokemonAbility}
          <RenderPokemonAbility {pokemonAbility} />
        {/each}
      </div>
    </div>
  </div>
</div>
