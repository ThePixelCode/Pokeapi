// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use pokeapi::Search;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn get_pokemon<'r>(
    search_options: pokeapi::search::SearchOption,
    store: tauri::State<'r, pokeapi::store::Store>,
) -> Result<pokeapi::pokemon::Pokemon, String> {
    if search_options == pokeapi::search::SearchOption::None {
        return Err(String::from("Unimplemented"));
    }

    let pokemon = pokeapi::pokemon::PokemonSearch::from(search_options);

    pokemon.search(store).await.map_err(|e| format!("{}", e))
}

fn main() {
    tauri::Builder::default()
        .manage(pokeapi::store::Store::default())
        .invoke_handler(tauri::generate_handler![greet, get_pokemon])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
