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
    #[error("IO Error {0}")]
    IOError(#[from] std::io::Error),
    #[error("Cannot Connect with API: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Lib Error: Requested source was not found")]
    NotFoundError,
    #[error("Lib Error: No cache dir fount")]
    NotCacheError,
}

pub trait Search {
    type Out;
    fn search<'r>(
        &self,
        store: tauri::State<'r, store::Store>,
    ) -> impl std::future::Future<Output = Result<Self::Out, Errors>> + Send;
}

pub fn get_contents_from_file<P>(path: P) -> Result<String, Errors>
where
    P: AsRef<std::path::Path>,
{
    use std::{fs::OpenOptions, io::Read};

    let mut file_contents = String::new();
    let mut file = OpenOptions::new()
        .create(false)
        .create_new(false)
        .read(true)
        .write(false)
        .open(path)?;

    file.read_to_string(&mut file_contents)?;

    Ok(file_contents)
}

pub fn replace_file_contents<P, C>(path: P, content: C) -> Result<(), Errors>
where
    P: AsRef<std::path::Path>,
    C: AsRef<[u8]>,
{
    use std::{fs::OpenOptions, io::Write};

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)?;

    file.write_all(content.as_ref())?;

    Ok(())
}

pub fn make_cache_files() -> Result<std::path::PathBuf, Errors> {
    use std::fs::create_dir_all;
    use tauri::api::path::cache_dir;

    let cache_dir = cache_dir()
        .ok_or(Errors::NotCacheError)?
        .join("thepixelcode/pokeapi");

    if !cache_dir.exists() || !cache_dir.is_dir() {
        create_dir_all(cache_dir.clone())?;
    }

    Ok(cache_dir)
}
