pub mod create_tree;
pub mod flatten;
pub mod osm_data;
pub mod parse;
pub mod parse_tags;
pub mod tag;

pub use flatten::flatten;
use osm_data::Nodes;
pub use osm_data::OsmData;
pub use parse::parse;
use tag::TagWithChildren;

use crate::osm::tag::TagTree;

#[tauri::command]
pub fn parse_osm(data: &str) -> OsmData {
    return parse(data);
}

#[tauri::command]
pub fn find_node_by_id(nodes: Nodes, id: &str) -> Option<Box<TagWithChildren>> {
    nodes
        .iter()
        .find(|node| node.parameters.get("id").unwrap() == id)
        .cloned()
}

#[tauri::command]
pub fn find_tag_by_name(tags: Nodes, name: &str) -> Option<Box<TagWithChildren>> {
    tags.iter().find(|tag| tag.tag_type == name).cloned()
}

#[tauri::command]
pub fn find_all_tags_by_name(tags: Nodes, name: &str) -> Option<TagTree> {
    let filtered_tags: Vec<Box<TagWithChildren>> = tags
        .iter()
        .filter(|tag| tag.tag_type == name)
        .cloned()
        .collect();

    return if filtered_tags.len() <= 0 {
        None
    } else {
        Some(filtered_tags)
    };
}

#[tauri::command]
pub fn get_all_nodes_of_way(way: TagWithChildren, nodes: Nodes) -> Option<TagTree> {
    if way.children.is_none() {
        return None;
    }

    let children = way.children.unwrap();

    Some(
        children
            .iter()
            .filter(|tag| tag.tag_type == "nd")
            .map(|nd| {
                nodes
                    .iter()
                    .find(|node| {
                        node.parameters.get("id").unwrap() == nd.parameters.get("ref").unwrap()
                    })
                    .unwrap()
            })
            .cloned()
            .collect(),
    )
}
