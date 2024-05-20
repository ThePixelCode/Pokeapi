pub mod pokemon;
pub mod search;
pub mod store;

const POKEAPI_BASE_URL: &str = "https://pokeapi.co/api/v2";

#[derive(Debug, Default, PartialEq, Eq, Clone, serde::Serialize, serde::Deserialize)]
pub struct NamedAPIResource {
    name: String,
    url: String,
}

#[derive(Debug, thiserror::Error)]
pub enum Errors {
    #[error("JSON Deserialize Error {0}")]
    JSONDeserializeError(#[from] serde_json::Error),
    #[error("Cannot Connect with API: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Requested source was not found")]
    NotFoundError,
}

pub trait Search {
    type Out;
    fn search<'r>(
        &self,
        store: tauri::State<'r, store::Store>,
    ) -> impl std::future::Future<Output = Result<Self::Out, Errors>> + Send;
}
