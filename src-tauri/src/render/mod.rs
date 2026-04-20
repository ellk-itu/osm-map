pub mod register_canvas;
pub mod render;

pub use register_canvas::register_canvas;
use tauri::ipc::Response;

use crate::{
    common::point::Point, osm::osm_data::get_public_osmdata, render::render::get_public_render,
};

#[tauri::command]
pub fn get_viewport_coords(node_ids: Vec<String>) -> Response {
    let render_guard = get_public_render();
    let render = render_guard.as_ref().unwrap();

    let osm_guard = get_public_osmdata();
    let osm = osm_guard.as_ref().unwrap();

    let mut points: Vec<Point<u16>> = vec![];

    for node_id in node_ids {
        let node = osm.nodes.get(&node_id).unwrap();

        points.push(render.translate_coordinates(node.lat, node.lon));
    }

    return Response::new(
        points
            .iter()
            .map(|p| p.to_le_bytes())
            .flatten()
            .flatten()
            .collect::<Vec<u8>>(),
    );
}

#[tauri::command]
pub fn get_viewport_coord(node_id: String) -> Response {
    let render_guard = get_public_render();
    let render = render_guard.as_ref().unwrap();

    let osm_guard = get_public_osmdata();
    let osm = osm_guard.as_ref().unwrap();

    let node = osm.nodes.get(&node_id).unwrap();
    let point = render.translate_coordinates(node.lat, node.lon);

    return Response::new(
        point
            .to_le_bytes()
            .iter()
            .flatten()
            .map(|n| *n)
            .collect::<Vec<u8>>(),
    );
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
