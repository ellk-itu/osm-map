use serde::{Deserialize, Serialize};

use crate::{
    common::point::Point,
    osm::osm_data::get_public_osmdata,
    render::render::{set_public_render, Render},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct CanvasParameters {
    width: u32,
    height: u32,
}

#[tauri::command]
pub fn register_canvas(params: CanvasParameters) {
    if let Some(osm_data) = get_public_osmdata().as_ref() {
        set_public_render(Render::new(osm_data, Point(params.width, params.height)));
    } else {
        panic!()
    }
}
