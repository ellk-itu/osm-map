pub mod osm_data;
pub mod parse;
pub mod tag;
pub mod tree;

use std::collections::HashMap;

pub use osm_data::OsmData;
pub use parse::parse;

use osm_data::set_public_osmdata;

use crate::osm::{osm_data::TagTagData, tag::Way, tree::Tag};

#[tauri::command]
pub fn parse_osm(data: &str) {
    set_public_osmdata(parse(data));
}

#[tauri::command]
pub fn get_ways() -> Vec<(Way, TagTagData)> {
    let guard = osm_data::get_public_osmdata();
    let osm = guard.as_ref().unwrap();

    osm.get_ways()
}

#[tauri::command]
pub fn get_osm_test() -> Option<OsmData> {
    return osm_data::get_public_osmdata().as_ref().cloned();
}

#[tauri::command]
pub fn get_tagtags(tag: Tag) -> HashMap<String, String> {
    let guard = osm_data::get_public_osmdata();
    let osm = guard.as_ref().unwrap();

    osm.get_tagtags(tag)
}
