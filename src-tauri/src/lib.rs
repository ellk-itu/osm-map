pub mod common;
pub mod osm;
pub mod render;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            osm::parse_osm,
            osm::get_osm_test,
            osm::get_ways,
            osm::get_tagtags,
            render::register_canvas::register_canvas,
            render::parse_ways,
            render::get_way_points
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
