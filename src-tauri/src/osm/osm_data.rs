use std::{
    collections::HashMap,
    sync::{LazyLock, RwLock, RwLockReadGuard},
};

use crate::{
    common::dev::TimedProcess,
    osm::{
        tag::{parse_tags, Bounds, Node, Osm, Relation, TagData, Way, Xml},
        tree::{self, TagTree},
    },
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
    pub(crate) xml: Xml,
    pub(crate) osm: Osm,
    pub(crate) bounds: Bounds,
    pub(crate) nodes: HashMap<String, Node>,
    pub(crate) ways: HashMap<String, Way>,
    pub(crate) relations: HashMap<String, Relation>,
}

impl OsmData {
    /// Parses osm data from file, creating the tree and returning a new `OsmData()`
    ///
    /// ## Parameters
    ///  * `file`: file in utf-8 format
    ///
    pub fn from_file(file: &str) -> OsmData {
        let osm_parsing = TimedProcess::start("Osm Parsing"); //Logging
        let tag_parsing = TimedProcess::start("Tag Parsing"); //Logging

        let parsed_tags = parse_tags(file);

        tag_parsing.stop();
        let tree_parsing = TimedProcess::start("Tree Parsing"); //Logging

        let tree = tree::from_tags(&parsed_tags);

        tree_parsing.stop();

        let mut xml: Option<Xml> = None;
        let mut osm: Option<Osm> = None;
        let mut bounds: Option<Bounds> = None;
        let mut nodes: HashMap<String, Node> = HashMap::new();
        let mut ways: HashMap<String, Way> = HashMap::new();
        let mut relations: HashMap<String, Relation> = HashMap::new();

        let specifics = TimedProcess::start("Specifics Parsing");

        if &parsed_tags[0].tag_type == "?xml" {
            xml = Some(Xml::new(parsed_tags[0].clone()));
        }

        if &parsed_tags[1].tag_type == "osm" {
            osm = Some(Osm::new(parsed_tags[1].clone()));
        }

        tree[1].1.iter().for_each(|tag| {
            let tag_data = parsed_tags[tag.0].clone();
            let children: Vec<TagData> =
                tag.1.iter().map(|tag| parsed_tags[tag.0].clone()).collect();

            match tag_data.tag_type.as_str() {
                "node" => {
                    let node = Node::new(tag_data.clone(), children);
                    nodes.insert(node.id.clone(), node);
                }
                "way" => {
                    let way = Way::new(tag_data, children);
                    ways.insert(way.id.clone(), way);
                }
                "relation" => {
                    let relation = Relation::new(tag_data, children);
                    relations.insert(relation.id.clone(), relation);
                }
                "bounds" => bounds = Some(Bounds::new(tag_data)),
                _ => panic!(),
            }
        });

        specifics.stop();

        let xml = xml.unwrap();
        let osm = osm.unwrap();
        let bounds = bounds.unwrap();

        osm_parsing.stop();

        return Self {
            xml,
            osm,
            bounds,
            nodes,
            ways,
            relations,
        };
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
