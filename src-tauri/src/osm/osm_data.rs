use std::{
    collections::HashMap,
    sync::{LazyLock, RwLock, RwLockReadGuard},
};

use crate::osm::{
    tag::{classify_tags, parse_tags, TagType, Way},
    tree::{self, Tag, TagTree},
};
use serde::{Deserialize, Serialize};

pub type Ways = TagTree;
pub type Nodes = TagTree;
pub type TagTagData = HashMap<String, String>;

/// Main struct containing all osm related data
///
/// ## Parameters
/// * `tags`: All of the raw data tags (see: `osm::tag::Tags`)
/// * `osm_tag`: The osm tag (see: `osm::tag::TagType::Osm`)
/// * `ways`: All the tags of type Way (see: `osm::tag::TagType::Way`),
/// * `nodes`: All the tags of type Node (see: `osm::tag::TagType::Node`),
///
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OsmData {
    pub(crate) tags: Vec<TagType>,
    pub(crate) osm_tag: Tag,

    pub(crate) ways: Ways,
    pub(crate) nodes: Nodes,

    pub(crate) node_by_id: HashMap<String, Tag>,
    pub(crate) ways_by_id: HashMap<String, Tag>,
}

impl OsmData {
    /// Gets the tag data from tag.
    ///
    /// returns the data from tag.
    ///
    /// ## Parameters
    ///  * `tag`: The tag to point to the data (see: `osm::tree::tag`)
    ///
    pub fn data<'a>(&'a self, tag: &Tag) -> &'a TagType {
        &self.tags[tag.0]
    }

    pub fn to_tags<'a>(
        &'a self,
        tags: &'a Vec<Box<Tag>>,
    ) -> impl Iterator<Item = &'a TagType> + 'a {
        tags.iter().map(move |tag| self.data(tag))
    }

    /// Parses osm data from file, creating the tree and returning a new `OsmData()`
    ///
    /// ## Parameters
    ///  * `file`: file in utf-8 format
    ///
    pub fn from_file(file: &str) -> OsmData {
        let parsed_tags = parse_tags(file);
        println!("Parsed tags");
        let tree = tree::from_tags(&parsed_tags);
        println!("created tree");
        let tags = classify_tags(&parsed_tags);
        println!("classefied tags");

        let osm_tag = tree
            .iter()
            .find(|tag| matches!(&tags[tag.0], TagType::Osm(_)))
            .unwrap()
            .clone();

        let nodes = osm_tag.filter_children(|tag| matches!(&tags[tag.0], TagType::Node(_)));
        let ways = osm_tag.filter_children(|tag| matches!(&tags[tag.0], TagType::Way(_)));

        let mut node_by_id: HashMap<String, Tag> = HashMap::new();

        for node in &nodes {
            let node_data = &tags[node.0];

            if let TagType::Node(data) = node_data {
                node_by_id.insert(data.id.clone(), *node.clone());
            }
        }

        let mut ways_by_id: HashMap<String, Tag> = HashMap::new();

        for way in &ways {
            let way_data = &tags[way.0];

            if let TagType::Node(data) = way_data {
                ways_by_id.insert(data.id.clone(), *way.clone());
            }
        }

        println!("Finished osm parsing");

        return OsmData {
            tags: tags.to_vec(),
            ways,
            nodes,
            osm_tag: *osm_tag,
            node_by_id,
            ways_by_id,
        };
    }

    pub fn get_ways(&self) -> Vec<(Way, TagTagData)> {
        println!("Getting ways");
        self.tags
            .iter()
            .filter_map(|tag| {
                if let TagType::Way(way) = tag {
                    Some(way.clone())
                } else {
                    None
                }
            })
            .map(|way| {
                let tag = self
                    .osm_tag
                    .find_child(|tag| {
                        let tag = self.data(tag);
                        if let TagType::Way(tag) = tag {
                            tag.id == way.id
                        } else {
                            false
                        }
                    })
                    .unwrap();
                (way, self.get_tagtags(*tag))
            })
            .collect::<Vec<(Way, TagTagData)>>()
    }

    pub fn get_tagtags(&self, tag: Tag) -> TagTagData {
        let mut tag_tags: TagTagData = HashMap::new();

        tag.1.iter().for_each(|tag| {
            let tag = self.data(tag);
            if let TagType::TagTag(tag_tag) = tag {
                tag_tags.insert(tag_tag.k.clone(), tag_tag.v.clone());
            }
        });

        return tag_tags;
    }
}

pub static OSMDATA: LazyLock<RwLock<Option<OsmData>>> = LazyLock::new(|| RwLock::new(None));

pub fn get_public_osmdata() -> RwLockReadGuard<'static, Option<OsmData>> {
    OSMDATA.read().expect("rwlock poisoned")
}

pub fn set_public_osmdata(data: OsmData) {
    let mut guard = OSMDATA.write().expect("rwlock poisoned");
    *guard = Some(data);
}
