use crate::osm::OsmData;

use super::{create_tree::create_tree, parse_tags::parse_tags};

pub fn parse(data: &str) -> OsmData {
    let parsed_tags = parse_tags(data);
    let tree = create_tree(parsed_tags);
    let osm_data = OsmData::from_tree(tree);

    return osm_data;
}
