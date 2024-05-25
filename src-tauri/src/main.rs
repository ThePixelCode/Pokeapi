// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use pokeapi_client::Search;
use tauri::Manager;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn get_pokemon<'r>(
    search_options: pokeapi_client::search::SearchOption,
    store: tauri::State<'r, pokeapi_client::store::Store>,
) -> Result<pokeapi_client::pokemon::Pokemon, String> {
    if search_options == pokeapi_client::search::SearchOption::None {
        return Err(String::from("Unimplemented"));
    }

    let pokemon = pokeapi_client::pokemon::PokemonSearch::from(search_options);

    pokemon.search(store).await.map_err(|e| format!("{}", e))
}

fn main() {
    tauri::Builder::default()
        .manage(pokeapi_client::store::Store::default())
        .invoke_handler(tauri::generate_handler![greet, get_pokemon])
        .on_window_event(|e| match e.event() {
            tauri::WindowEvent::CloseRequested { .. } => {
                let store = e.window().state::<pokeapi_client::store::Store>();
                store.save_to_disk();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
