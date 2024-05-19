use std::{collections::HashMap, sync::Mutex};

#[derive(Debug, Default)]
pub struct Store {
    pub pokemon: Mutex<HashMap<u32, String>>,
}
