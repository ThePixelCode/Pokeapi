const POKEMON_URL: &str = "/pokemon";

#[derive(Debug, Default, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub struct PokemonAbility {
    is_hidden: bool,
    slot: u32,
    ability: crate::NamedAPIResource,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub struct PokemonHeldItemVersion {
    rarity: u32,
    version: crate::NamedAPIResource,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub struct PokemonHeldItem {
    item: crate::NamedAPIResource,
    version_details: Vec<PokemonHeldItemVersion>,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub struct PokemonMoveVersion {
    level_learned_at: u32,
    move_learn_method: crate::NamedAPIResource,
    version_group: crate::NamedAPIResource,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub struct PokemonMove {
    #[serde(rename = "move")]
    move_field: crate::NamedAPIResource,
    version_group_details: Vec<PokemonMoveVersion>,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub struct PokemonStat {
    stat: crate::NamedAPIResource,
    effort: u32,
    base_stat: u32,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub struct PokemonType {
    #[serde(rename = "type")]
    type_field: crate::NamedAPIResource,
    slot: u32,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub struct PokemonSprites {
    front_default: String,
    front_shiny: String,
    front_female: String,
    front_shiny_female: String,
    back_default: String,
    back_shiny: String,
    back_female: String,
    back_shiny_female: String,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub struct Pokemon {
    id: u32,
    name: String,
    base_expirence: u32,
    height: u32,
    weight: u32,
    abilities: Vec<PokemonAbility>,
    moves: Vec<PokemonMove>,
    species: crate::NamedAPIResource,
    stats: Vec<PokemonStat>,
    types: Vec<PokemonType>,
    sprites: PokemonSprites,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub struct PokemonSearch {
    option: Option<crate::search::SearchOption>,
}

impl From<crate::search::SearchOption> for PokemonSearch {
    fn from(value: crate::search::SearchOption) -> Self {
        Self {
            option: Some(value),
        }
    }
}

impl crate::search::NamedSearch for PokemonSearch {
    fn set_option(mut self, option: crate::search::SearchOption) -> Self {
        self.option = Some(option);
        self
    }

    fn get_option(&self) -> Option<&crate::search::SearchOption> {
        self.option.as_ref()
    }
}

impl PokemonSearch {
    pub fn search(&self) -> Pokemon {
        todo!()
    }
}
