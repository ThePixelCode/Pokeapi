<script lang="ts">
    import type { PokemonStat } from "../types/pokemon";

    export let pokemonStat: PokemonStat;
</script>

<div class="grid grid-cols-3">
    <h1>{pokemonStat.stat.name}:</h1>
    <h1>Base stat: {pokemonStat.base_stat}</h1>
    <h1>Effort: {pokemonStat.effort}</h1>
</div>
