pub mod osm_parser;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn parse_osm(data: &str) -> Vec<osm_parser::Tag> {
    return osm_parser::parse(data);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![parse_osm])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
