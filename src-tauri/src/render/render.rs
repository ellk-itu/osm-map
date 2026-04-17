use std::{
    collections::HashMap,
    sync::{LazyLock, RwLock, RwLockReadGuard},
};

use serde::{Deserialize, Serialize};

use crate::{
    common::point::Point,
    osm::{
        osm_data::get_public_osmdata,
        tag::{TagType, Way},
        tree::Tag,
        OsmData,
    },
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Render {
    pub(crate) viewport_size: Point<u32>,
    pub(crate) coord_offset: Point<f64>,
    pub(crate) min_bounds: Point<f64>,
    pub(crate) max_bounds: Point<f64>,
}

impl Render {
    pub fn new(osm_data: &OsmData, viewport_size: Point<u32>) -> Render {
        let bounds = osm_data.data(
            &osm_data
                .osm_tag
                .find_child(|tag| matches!(osm_data.data(tag), TagType::Bounds(_)))
                .unwrap(),
        );

        if let TagType::Bounds(bounds) = bounds {
            let coord_offset = Point(
                bounds.max_lon - bounds.min_lon,
                bounds.max_lat - bounds.min_lat,
            );

            let min_bounds = Point::<f64>(bounds.min_lon, bounds.min_lat);
            let max_bounds = Point::<f64>(bounds.max_lon, bounds.max_lat);

            return Render {
                viewport_size,
                coord_offset,
                min_bounds,
                max_bounds,
            };
        } else {
            panic!();
        }
    }

    fn viewport_scale(&self) -> Point<f64> {
        Point(
            self.viewport_size.0 as f64 / self.coord_offset.0,
            self.viewport_size.1 as f64 / self.coord_offset.1,
        )
    }

    pub fn translate_coordinates(&self, lat: f64, lon: f64) -> Point<f64> {
        Point(
            (lon - self.min_bounds.0) * self.viewport_scale().0,
            (-(lat - self.min_bounds.1) * self.viewport_scale().1) + self.viewport_size.1 as f64,
        )
    }

    pub fn parse_way_points(&self, osm: &OsmData) {
        let mut coords: HashMap<String, Vec<u16>> = HashMap::with_capacity(osm.ways.len());
        for way in &osm.ways {
            if let TagType::Way(data) = osm.data(&way) {
                coords.insert(data.id.clone(), self.parse_way(&way));
            }
        }

        set_way_coords(coords);
    }

    fn parse_way(&self, way: &Tag) -> Vec<u16> {
        let guard = get_public_osmdata();
        let osm = guard.as_ref().unwrap();

        let nd_vec: Vec<String> = osm
            .to_tags(&way.1)
            .filter_map(|tag| {
                if let TagType::Nd(t) = tag {
                    return Some(t.r#ref.clone());
                } else {
                    return None;
                }
            })
            .collect();

        let nodes = nd_vec
            .iter()
            .map(|id| osm.node_by_id.get(id).unwrap())
            .map(|tag| osm.data(tag));

        let coords: Vec<Point<f64>> = nodes
            .filter_map(|tag| {
                if let TagType::Node(node) = tag {
                    Some(self.translate_coordinates(node.lat, node.lon))
                } else {
                    None
                }
            })
            .collect();

        let mut mapped_coords: Vec<u16> = vec![];
        coords.iter().for_each(|coord| {
            mapped_coords.push(coord.0 as u16);
            mapped_coords.push(coord.1 as u16);
        });

        return mapped_coords;
    }
}

pub fn get_way_points(id: String) -> Option<Vec<u16>> {
    let guard = get_way_coords();
    let way_coords = guard.as_ref().unwrap();
    way_coords.get(&id).cloned()
}

static WAYCOORDS: LazyLock<RwLock<Option<HashMap<String, Vec<u16>>>>> =
    LazyLock::new(|| RwLock::new(None));

pub fn get_way_coords() -> RwLockReadGuard<'static, Option<HashMap<String, Vec<u16>>>> {
    WAYCOORDS.read().expect("rwlock poisoned")
}

fn set_way_coords(data: HashMap<String, Vec<u16>>) {
    let mut guard = WAYCOORDS.write().expect("rwlock poisoned");
    *guard = Some(data);
}

pub static RENDER: LazyLock<RwLock<Option<Render>>> = LazyLock::new(|| RwLock::new(None));

pub fn get_public_render() -> RwLockReadGuard<'static, Option<Render>> {
    RENDER.read().expect("rwlock poisoned")
}

pub fn set_public_render(data: Render) {
    let mut guard = RENDER.write().expect("rwlock poisoned");
    *guard = Some(data);
}
