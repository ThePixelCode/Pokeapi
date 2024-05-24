<script lang="ts">
    import type { PokemonType } from "../types/pokemon";
    import { capitalize_first } from "../utils";

    export let pokemonType: PokemonType;
</script>

<h1>
    Type {capitalize_first(pokemonType.type.name)} on slot {pokemonType.slot}
</h1>
