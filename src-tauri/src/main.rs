// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn greet(name:&str) -> String{
  format!("Hello {name}")
}

use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};

fn main() {
  let close = CustomMenuItem::new("close".to_string(), "Close");
  let submenu = Submenu::new("File", Menu::new().add_item(close));
  let menu = Menu::new()
      .add_native_item(MenuItem::Copy)
      .add_submenu(submenu);

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet])
    .menu(menu)
    .on_menu_event(|event| {
        match event.menu_item_id() {
            "close" => {
                std::process::exit(0);
            }
            _ => {}
        }
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

}
