pub mod osm_data;
pub mod parse;
pub mod tag;
pub mod tree;

use std::collections::HashMap;

pub use osm_data::OsmData;
pub use parse::parse;

use osm_data::set_public_osmdata;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::osm::tag::{Node, Way};

#[tauri::command]
pub fn parse_osm(data: &str) {
    set_public_osmdata(parse(data));
}

type Tags = HashMap<String, String>;

#[tauri::command]
pub fn get_ways(way_ids: Option<Vec<String>>) -> HashMap<String, Way> {
    let guard = osm_data::get_public_osmdata();
    let osm = guard.as_ref().unwrap();

    if way_ids.is_none() {
        return osm.ways.clone();
    }

    let way_ids = way_ids.unwrap();
    let mut return_arr: HashMap<String, Way> = HashMap::new();

    for id in way_ids {
        let way = osm.ways.get(&id).unwrap();
        return_arr.insert(id.clone(), way.clone());
    }

    return return_arr;
}

#[tauri::command]
pub fn get_sorted_ways() -> Vec<Vec<String>> {
    let sort_conditions: Vec<(Box<dyn Fn(&Tags) -> bool>, usize)> = vec![
        (
            Box::new(|t| {
                if let Some(place) = t.get("place") {
                    place == "island"
                } else {
                    false
                }
            }),
            0,
        ),
        (
            Box::new(|t| {
                if let Some(landuse) = t.get("landuse") {
                    landuse == "residential"
                } else {
                    false
                }
            }),
            1,
        ),
        (
            Box::new(|t| {
                if let Some(landuse) = t.get("landuse") {
                    landuse == "Basin"
                } else {
                    false
                }
            }),
            1,
        ),
        (
            Box::new(|t| {
                if let Some(landuse) = t.get("landuse") {
                    landuse == "farmland"
                } else {
                    false
                }
            }),
            1,
        ),
        (
            Box::new(|t| {
                if let Some(landuse) = t.get("landuse") {
                    landuse == "meadow"
                } else {
                    false
                }
            }),
            1,
        ),
        (
            Box::new(|t| {
                if let Some(natural) = t.get("natural") {
                    natural == "wood"
                } else {
                    false
                }
            }),
            2,
        ),
        (
            Box::new(|t| {
                if let Some(natural) = t.get("natural") {
                    natural == "water"
                } else {
                    false
                }
            }),
            2,
        ),
        (
            Box::new(|t| {
                if let Some(building) = t.get("building") {
                    building == "yes"
                } else {
                    false
                }
            }),
            3,
        ),
        (Box::new(|_| true), 15),
    ];

    let guard = osm_data::get_public_osmdata();
    let osm = guard.as_ref().unwrap();
    let mut sorted_ways: Vec<Vec<String>> = vec![];
    sorted_ways.resize_with(16, || vec![]);

    osm.ways.iter().for_each(|(_, way)| {
        for condition in &sort_conditions {
            if condition.0(&way.tags) {
                sorted_ways[condition.1].push(way.id.clone());
                break;
            }
        }
    });

    return sorted_ways;
}

#[tauri::command]
pub fn get_nodes(node_ids: Option<Vec<String>>) -> HashMap<String, Node> {
    let guard = osm_data::get_public_osmdata();
    let osm = guard.as_ref().unwrap();

    if node_ids.is_none() {
        return osm.nodes.clone();
    }

    let node_ids = node_ids.unwrap();
    let mut return_arr: HashMap<String, Node> = HashMap::new();

    for id in node_ids {
        let node = osm.nodes.get(&id).unwrap();
        return_arr.insert(id.clone(), node.clone());
    }

    return return_arr;
}

#[tauri::command]
pub fn get_osm_test() -> Option<OsmData> {
    return osm_data::get_public_osmdata().as_ref().cloned();
}
