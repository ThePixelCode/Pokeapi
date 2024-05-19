pub mod search;

const POKEAPI_BASE_URL: &str = "https://pokeapi.co/api/v2";

#[derive(Debug, Default, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub struct NamedAPIResource {
    name: String,
    url: String,
}
