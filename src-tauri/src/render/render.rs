use std::{
    collections::HashMap,
    sync::{LazyLock, RwLock, RwLockReadGuard},
};

use serde::{Deserialize, Serialize};

use crate::{
    common::point::Point,
    osm::{
        osm_data::{self, get_public_osmdata},
        tag::Way,
        tree::TagRef,
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
        let bounds = &osm_data.bounds;

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
    }

    fn viewport_scale(&self) -> Point<f64> {
        Point(
            self.viewport_size.0 as f64 / self.coord_offset.0,
            self.viewport_size.1 as f64 / self.coord_offset.1,
        )
    }

    pub fn translate_coordinates(&self, lat: f64, lon: f64) -> Point<u16> {
        Point(
            ((lon - self.min_bounds.0) * self.viewport_scale().0) as u16,
            ((-(lat - self.min_bounds.1) * self.viewport_scale().1) + self.viewport_size.1 as f64)
                as u16,
        )
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
