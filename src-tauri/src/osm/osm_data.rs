use crate::osm::tag::{TagTree, TagWithChildren};
use serde::{Deserialize, Serialize};

pub type Ways = TagTree;
pub type Nodes = TagTree;

#[derive(Debug, Serialize, Deserialize)]
pub struct OsmData {
    pub(crate) ways: Ways,
    pub(crate) nodes: Nodes,
    pub(crate) tree: TagTree,
    pub(crate) bounds: TagWithChildren,
}

impl OsmData {
    pub fn from_tree(tree: TagTree) -> OsmData {
        let osm_tag = tree.iter().find(|tag| tag.tag_type == "osm").unwrap();

        let bounds = *osm_tag.find(|tag| tag.tag_type == "bounds").unwrap();
        let nodes = osm_tag.find_all(|tag| tag.tag_type == "node");
        let ways = osm_tag.find_all(|tag| tag.tag_type == "way");

        return OsmData {
            ways,
            nodes,
            tree,
            bounds,
        };
    }
}
