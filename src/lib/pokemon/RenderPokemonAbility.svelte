<script lang="ts">
    import type { PokemonAbility } from "../types/pokemon";
    import { capitalize_first } from "../utils";

    export let pokemonAbility: PokemonAbility;
</script>

<div class="grid grid-cols-3">
    <h1>{capitalize_first(pokemonAbility.ability.name)}</h1>
    <h1>Is hidden: {pokemonAbility.is_hidden ? "yes" : "no"}</h1>
    <h1>Slot: {pokemonAbility.slot}</h1>
</div>
