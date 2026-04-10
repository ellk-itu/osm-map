pub mod common;
pub mod osm;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            osm::parse_osm,
            osm::find_node_by_id,
            osm::find_tag_by_name,
            osm::find_all_tags_by_name,
            osm::get_all_nodes_of_way
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
