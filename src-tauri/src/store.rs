use std::{collections::HashMap, sync::Mutex};

#[derive(Debug, Default)]
pub struct Store {
    pub pokemon: Mutex<HashMap<u32, String>>,
    pub pokemon_ids: Mutex<HashMap<String, u32>>,
}
