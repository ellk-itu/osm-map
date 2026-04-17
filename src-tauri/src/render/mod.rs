pub mod register_canvas;
pub mod render;

pub use register_canvas::register_canvas;
use tauri::ipc::Response;

use crate::{osm::osm_data::get_public_osmdata, render::render::get_public_render};

#[tauri::command]
pub fn parse_ways() {
    let render_guard = get_public_render();
    let render = render_guard.as_ref().unwrap();

    let osm_guard = get_public_osmdata();
    let osm = osm_guard.as_ref().unwrap();

    render.parse_way_points(osm);
}

#[tauri::command]
pub fn get_way_points(id: String) -> Response {
    let coords = render::get_way_points(id);

    if let Some(coords) = coords {
        let points: Vec<u8> = coords
            .iter()
            .flat_map(|n| n.to_le_bytes().into_iter())
            .collect();

        return tauri::ipc::Response::new(points);
    }

    return tauri::ipc::Response::new(Vec::new());
}

// #[tauri::command]
// pub fn get_way() -> Response {
//     let osm_guard = get_public_osmdata();
//     let osm = osm_guard.as_ref().unwrap();

//     let render_guard = get_public_render();
//     let render = render_guard.as_ref().unwrap();

//     let points: Vec<u8> = render
//         .get_way_points(&osm.ways[0])
//         .iter()
//         .flat_map(|n| n.to_le_bytes().into_iter())
//         .collect();

//     return tauri::ipc::Response::new(points);
// }
