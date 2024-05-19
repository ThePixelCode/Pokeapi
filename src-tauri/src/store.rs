use std::{collections::HashMap, sync::Mutex};

#[derive(Debug, Default)]
pub struct Store {
    pokemon: Mutex<HashMap<u32, String>>,
}
