import type { RemoteURIResource } from "./generic";

export type PokemonAbility = {
    is_hidden: boolean,
    slot: number,
    ability: RemoteURIResource,
};

export type PokemonMove = {
    move: RemoteURIResource,
    version_group_details: {
        level_learned_at: number,
        move_learn_method: RemoteURIResource,
        version_group: RemoteURIResource,
    }[],
};

export type PokemonSprite = {
    front_default: string,
    front_shiny: string,
    front_female: string | null,
    front_shiny_female: string | null,
    back_default: string,
    back_shiny: string,
    back_female: string | null,
    back_shiny_female: string | null,
};

export type PokemonStat = {
    stat: RemoteURIResource,
    effort: number,
    base_stat: number,
};

export type PokemonType = {
    type: RemoteURIResource,
    slot: number,
};

export type Pokemon = {
    id: number,
    name: string,
    base_experience: number,
    height: number,
    weight: number,
    abilities: PokemonAbility[],
    moves: PokemonMove[],
    species: RemoteURIResource,
    stats: PokemonStat[],
    types: PokemonType[],
    sprites: PokemonSprite,
}