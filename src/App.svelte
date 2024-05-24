<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import Header from "./lib/Header.svelte";
  import Search from "./lib/Search.svelte";
  import RenderPokemon from "./lib/pokemon/RenderPokemon.svelte";
  import type { Pokemon } from "./lib/types/pokemon";

  let pokemon: Pokemon | undefined = undefined;
</script>

<Header />

<div class="grid grid-cols-2">
  <div>
    <Search
      callback={async (val) => {
        pokemon = await invoke("get_pokemon", { searchOptions: val });
      }}
      searchFor="Pokemon"
    />
  </div>
  {#if pokemon !== undefined}
    <div class="bg-neutral-200 dark:bg-neutral-700 m-5">
      <RenderPokemon {pokemon} />
    </div>
  {/if}
</div>
