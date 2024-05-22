use std::{collections::HashMap, sync::Mutex};

#[derive(Debug)]
pub struct Store {
    pub pokemon: Mutex<HashMap<u32, String>>,
    pub pokemon_ids: Mutex<HashMap<String, u32>>,
}

impl Default for Store {
    fn default() -> Self {
        use serde_json::from_str;

        let cache_dir = crate::make_cache_files();

        if cache_dir.is_err() {
            eprintln!("Cannot create cache dirs... ignoring");
            return Self {
                pokemon: Default::default(),
                pokemon_ids: Default::default(),
            };
        }

        let cache_dir = cache_dir.expect("Err is unreachable");

        let cached_pokemon = crate::get_contents_from_file(cache_dir.join("pokemon.json"));
        let cached_pokemon_ids = crate::get_contents_from_file(cache_dir.join("pokemon_ids.json"));

        if cached_pokemon.is_err() || cached_pokemon_ids.is_err() {
            eprintln!("Cannot read cached files... ignoring");
            return Self {
                pokemon: Default::default(),
                pokemon_ids: Default::default(),
            };
        }

        let cached_pokemon = cached_pokemon.expect("Err is unreachable");
        let cached_pokemon_ids = cached_pokemon_ids.expect("Err is unreachable");

        let cached_pokemon = from_str::<HashMap<u32, String>>(&cached_pokemon);
        let cached_pokemon_ids = from_str::<HashMap<String, u32>>(&cached_pokemon_ids);

        if cached_pokemon.is_err() || cached_pokemon_ids.is_err() {
            eprintln!("Cannot read cached files... ignoring");
            return Self {
                pokemon: Default::default(),
                pokemon_ids: Default::default(),
            };
        }

        let cached_pokemon = cached_pokemon.expect("Err is unreachable");
        let cached_pokemon_ids = cached_pokemon_ids.expect("Err is unreachable");

        eprintln!("cache restored, using it... if this fails try removing cache dir");

        Self {
            pokemon: Mutex::new(cached_pokemon),
            pokemon_ids: Mutex::new(cached_pokemon_ids),
        }
    }
}

impl Store {
    pub fn save_to_disk(&self) {
        use serde_json::to_string;

        let cache_dir = crate::make_cache_files();

        if cache_dir.is_err() {
            return;
        }

        let cache_dir = cache_dir.expect("Err is unreachable");

        let cached_pokemon = self.pokemon.lock();
        let cached_pokemon_ids = self.pokemon_ids.lock();

        if cached_pokemon.is_err() || cached_pokemon_ids.is_err() {
            return;
        }

        let cached_pokemon = cached_pokemon.expect("Err is unreachable");
        let cached_pokemon_ids = cached_pokemon_ids.expect("Err is unreachable");

        let cached_pokemon = to_string(&*cached_pokemon).expect("Err is unreachable");
        let cached_pokemon_ids = to_string(&*cached_pokemon_ids).expect("Err is unreachable");

        let _ = crate::replace_file_contents(cache_dir.join("pokemon.json"), cached_pokemon);
        let _ =
            crate::replace_file_contents(cache_dir.join("pokemon_ids.json"), cached_pokemon_ids);
        eprintln!("cache successfully save");
    }
}
