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
    front_female: Option<String>,
    front_shiny_female: Option<String>,
    back_default: String,
    back_shiny: String,
    back_female: Option<String>,
    back_shiny_female: Option<String>,
}

#[derive(Debug, Default, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub struct Pokemon {
    id: u32,
    name: String,
    base_experience: u32,
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

use crate::search::NamedSearch;
impl crate::Search for PokemonSearch {
    type Out = Pokemon;
    async fn search<'r>(
        &self,
        store: tauri::State<'r, crate::store::Store>,
    ) -> Result<Self::Out, crate::Errors> {
        use serde_json::{from_str, to_string};
        if self.get_option().is_none()
            || self
                .get_option()
                .is_some_and(|op| *op == crate::search::SearchOption::None)
        {
            unimplemented!()
        }

        let mut pokemon_string = None;

        {
            let cached_pokemons = store.pokemon.lock().expect(
                "Mutex is poisoned, which completly breaks the universe this app is suppose to work",
            );
            let cached_ids = store.pokemon_ids.lock().expect(
                "Mutex is poisoned, which completly breaks the universe this app is suppose to work",
            );

            let id = if let Some(name) = self.get_name() {
                cached_ids.get(name)
            } else {
                self.get_id()
            };

            if let Some(id) = id {
                if cached_pokemons.contains_key(id) {
                    pokemon_string = Some(
                        cached_pokemons
                            .get(id)
                            .expect("None is unreachable")
                            .clone(),
                    );
                }
            }
        }

        if let Some(pokemon_string) = pokemon_string {
            eprintln!("found on the cache!");
            return Ok(from_str::<Pokemon>(&pokemon_string)?);
        }

        let pokemon = if let Some(id) = self.get_id() {
            crate::search::get(format!("{}{}/{}", crate::POKEAPI_BASE_URL, POKEMON_URL, id)).await?
        } else {
            crate::search::get(format!(
                "{}{}/{}",
                crate::POKEAPI_BASE_URL,
                POKEMON_URL,
                self.get_name().expect("None should be unreachable")
            ))
            .await?
        };

        eprintln!("cache missed searching on server");

        let pokemon = from_str::<Pokemon>(&pokemon)?;

        {
            let mut cached_pokemon = store.pokemon.lock().expect(
                "Mutex is poisoned, which completly breaks the universe this app is suppose to work",
            );
            cached_pokemon.insert(pokemon.id.clone(), to_string(&pokemon)?);

            let mut cached_ids = store.pokemon_ids.lock().expect(
                "Mutex is poisoned, which completly breaks the universe this app is suppose to work",
            );
            cached_ids.insert(pokemon.name.clone(), pokemon.id.clone());
        }

        Ok(pokemon)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ensure_async() {
        fn ensure<T>(_item: T)
        where
            T: Sync + Send,
        {
        }
        ensure(Pokemon::default());
    }
}
