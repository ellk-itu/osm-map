use crate::osm::OsmData;

/// Parses osm file
///
/// returns new `OsmData`
///
/// ## Parameters
///  * `data`: osm file
///
pub fn parse(data: &str) -> OsmData {
    OsmData::from_file(data)
}
