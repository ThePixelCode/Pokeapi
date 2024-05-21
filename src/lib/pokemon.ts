type Pokemon = {
    id: number,
    name: string,
    base_experience: number,
    height: number,
    weight: number,
    abilities: {
        is_hidden: boolean,
        slot: number,
        ability: {
            name: string,
            url: string,
        },
    }[],
    moves: {
        move: {
            name: string,
            url: string,
        },
        version_group_details: {
            level_learned_at: number,
            move_learn_method: {
                name: string,
                url: string,
            },
            version_group: {
                name: string,
                url: string,
            },
        }[],
    }[],
    species: {
        name: string,
        url: string,
    },
    stats: {
        stat: {
            name: string,
            url: string,
        },
        effort: number,
        base_stat: number,
    }[],
    types: {
        type: {
            name: string,
            url: string,
        },
        slot: number,
    }[],
    sprites: {
        front_default: string,
        front_shiny: string,
        front_female: string | null,
        front_shiny_female: string | null,
        back_default: string,
        back_shiny: string,
        back_female: string | null,
        back_shiny_female: string | null,
    },
}