use crate::common::str_tools::{remove_first, remove_last};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagData {
    pub(crate) tag_type: String,
    pub(crate) parameters: HashMap<String, String>,
    pub(crate) is_end_tag: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bounds {
    pub(crate) min_lat: f64,
    pub(crate) max_lat: f64,
    pub(crate) min_lon: f64,
    pub(crate) max_lon: f64,
}

impl Bounds {
    pub fn new(data: TagData) -> Self {
        return Self {
            min_lat: data
                .parameters
                .get("minlat")
                .unwrap()
                .parse::<f64>()
                .unwrap(),
            max_lat: data
                .parameters
                .get("maxlat")
                .unwrap()
                .parse::<f64>()
                .unwrap(),
            min_lon: data
                .parameters
                .get("minlon")
                .unwrap()
                .parse::<f64>()
                .unwrap(),
            max_lon: data
                .parameters
                .get("maxlon")
                .unwrap()
                .parse::<f64>()
                .unwrap(),
        };
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub(crate) id: String,
    pub(crate) lon: f64,
    pub(crate) lat: f64,
    pub(crate) user: String,
    pub(crate) changeset: String,
    pub(crate) version: String,
    pub(crate) timestamp: String,
    pub(crate) visible: Option<bool>,
    pub(crate) uid: String,
    pub(crate) tags: HashMap<String, String>,
}

impl Node {
    pub fn new(data: TagData, children: Vec<TagData>) -> Self {
        let mut tags: HashMap<String, String> = HashMap::new();

        for child in children {
            if child.tag_type == "tag" {
                tags.insert(
                    child.parameters.get("k").unwrap().clone(),
                    child.parameters.get("v").unwrap().clone(),
                );
            }
        }

        return Self {
            id: data.parameters.get("id").unwrap().clone(),
            lon: data.parameters.get("lon").unwrap().parse::<f64>().unwrap(),
            lat: data.parameters.get("lat").unwrap().parse::<f64>().unwrap(),
            user: data.parameters.get("user").unwrap().clone(),
            changeset: data.parameters.get("changeset").unwrap().clone(),
            version: data.parameters.get("version").unwrap().clone(),
            timestamp: data.parameters.get("timestamp").unwrap().clone(),
            visible: if let Some(b) = data.parameters.get("visible") {
                Some(b.parse::<bool>().unwrap())
            } else {
                None
            },
            uid: data.parameters.get("uid").unwrap().clone(),
            tags,
        };
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Way {
    pub(crate) id: String,
    pub(crate) user: String,
    pub(crate) changeset: String,
    pub(crate) version: String,
    pub(crate) timestamp: String,
    pub(crate) visible: Option<bool>,
    pub(crate) uid: String,
    pub(crate) tags: HashMap<String, String>,
    pub(crate) node_refs: Vec<String>,
}

impl Way {
    pub fn new(data: TagData, children: Vec<TagData>) -> Self {
        let mut tags: HashMap<String, String> = HashMap::new();
        let mut node_refs: Vec<String> = vec![];

        for child in children {
            if child.tag_type == "tag" {
                tags.insert(
                    child.parameters.get("k").unwrap().clone(),
                    child.parameters.get("v").unwrap().clone(),
                );
                continue;
            }

            if child.tag_type == "nd" {
                node_refs.push(child.parameters.get("ref").unwrap().clone());
                continue;
            }
        }

        return Self {
            id: data.parameters.get("id").unwrap().clone(),
            user: data.parameters.get("user").unwrap().clone(),
            changeset: data.parameters.get("changeset").unwrap().clone(),
            version: data.parameters.get("version").unwrap().clone(),
            timestamp: data.parameters.get("timestamp").unwrap().clone(),
            visible: if let Some(b) = data.parameters.get("visible") {
                Some(b.parse::<bool>().unwrap())
            } else {
                None
            },
            uid: data.parameters.get("uid").unwrap().clone(),
            tags,
            node_refs,
        };
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemberType {
    Node,
    Way,
    Relation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    pub(crate) role: String,
    pub(crate) r#type: MemberType,
    pub(crate) r#ref: String,
}

impl Member {
    pub fn new(data: TagData) -> Self {
        let r#type = match data.parameters.get("type").unwrap().as_str() {
            "node" => MemberType::Node,
            "way" => MemberType::Way,
            "relation" => MemberType::Relation,
            _ => panic!(
                "unknown type {}",
                data.parameters.get("type").unwrap().as_str()
            ),
        };

        return Self {
            role: data.parameters.get("role").unwrap().clone(),
            r#ref: data.parameters.get("ref").unwrap().clone(),
            r#type,
        };
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    pub(crate) id: String,
    pub(crate) user: String,
    pub(crate) changeset: String,
    pub(crate) version: String,
    pub(crate) timestamp: String,
    pub(crate) visible: Option<bool>,
    pub(crate) uid: String,
    pub(crate) members: Vec<Member>,
    pub(crate) tags: HashMap<String, String>,
}

impl Relation {
    pub fn new(data: TagData, children: Vec<TagData>) -> Self {
        let mut tags: HashMap<String, String> = HashMap::new();
        let mut members: Vec<Member> = vec![];

        for child in children {
            if child.tag_type == "tag" {
                tags.insert(
                    child.parameters.get("k").unwrap().clone(),
                    child.parameters.get("v").unwrap().clone(),
                );
                continue;
            }

            if child.tag_type == "member" {
                members.push(Member::new(child));
                continue;
            }
        }

        return Self {
            id: data.parameters.get("id").unwrap().clone(),
            user: data.parameters.get("user").unwrap().clone(),
            changeset: data.parameters.get("changeset").unwrap().clone(),
            version: data.parameters.get("version").unwrap().clone(),
            timestamp: data.parameters.get("timestamp").unwrap().clone(),
            visible: if let Some(b) = data.parameters.get("visible") {
                Some(b.parse::<bool>().unwrap())
            } else {
                None
            },
            uid: data.parameters.get("uid").unwrap().clone(),
            tags,
            members,
        };
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Osm {
    pub(crate) version: String,
    pub(crate) copyright: Option<String>,
    pub(crate) attribution: Option<String>,
    pub(crate) license: Option<String>,
}

impl Osm {
    pub fn new(data: TagData) -> Self {
        return Self {
            version: data.parameters.get("version").unwrap().clone(),
            copyright: data.parameters.get("copyright").cloned(),
            attribution: data.parameters.get("attribution").cloned(),
            license: data.parameters.get("license").cloned(),
        };
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Xml {
    pub(crate) version: String,
    pub(crate) encoding: String,
}

impl Xml {
    pub fn new(data: TagData) -> Self {
        return Self {
            version: data.parameters.get("version").unwrap().clone(),
            encoding: data.parameters.get("version").unwrap().clone(),
        };
    }
}

/// Parses file into tags.
///
/// Returns a list of all the parsed tags (see: `TagData`)
///
/// ## Parameters
///  * `data`: Osm data file
///
pub fn parse_tags(data: &str) -> Vec<TagData> {
    let tags: Vec<&str> = data.trim().split('>').collect();
    let parsed_tags: Vec<TagData> = tags.par_iter().map(|tag| parse_tag(tag)).collect();

    return parsed_tags;
}

fn remove_end_tag_symbol(string: &str) -> &str {
    if string.len() <= 0 {
        return string;
    }

    if &string[0..1] == "/" {
        return remove_first(string);
    } else if &string[string.len() - 1..] == "/" {
        return remove_last(string);
    }

    return string;
}

/// String split by whitespace but preserves everything in between "" symbols.
fn custom_split(string: &str) -> Vec<String> {
    let mut strings: Vec<String> = vec![];
    let mut split_string: String = String::new();
    let mut quotation_char: Option<char> = None;

    for character in string.chars() {
        if character == ' ' && quotation_char.is_none() {
            strings.push(split_string.clone());
            split_string = String::new();
            continue;
        }

        if (character == '\'' || character == '"') && quotation_char.is_none() {
            quotation_char = Some(character);
            split_string.push(character);
            continue;
        }

        if !quotation_char.is_none() {
            if character == quotation_char.unwrap() {
                quotation_char = None
            }
        }

        split_string.push(character);
    }

    // Add the last part
    strings.push(split_string);

    return strings;
}

fn is_end_tag(string: &str) -> bool {
    if string.len() <= 0 {
        return false;
    }

    return &string[0..1] == "/" || &string[string.len() - 1..] == "/" || &string[0..1] == "?";
    // Fuckass case where xml is ?
}

fn parse_tag(tag: &str) -> TagData {
    // Remove the < symbol
    let first_char_trimmed = remove_first(tag.trim());

    // Get and remove the end tag symbol
    let is_end_tag = is_end_tag(first_char_trimmed);
    let removed_end_symbol = remove_end_tag_symbol(first_char_trimmed);

    // Split into parameters
    let split_tag = custom_split(removed_end_symbol);

    let mut tag_type: Option<String> = None;
    let mut parameters: HashMap<String, String> = HashMap::with_capacity(split_tag.len() - 1);

    for (i, param) in split_tag.iter().enumerate() {
        if i == 0 {
            tag_type = Some(param.clone());
            continue;
        }

        let key_value: Vec<&str> = param.split("=").collect();

        // Remove the " chars
        let first_removed = remove_first(key_value[1]);
        let last_removed = remove_last(first_removed);

        parameters.insert(key_value[0].to_string(), last_removed.to_string());
    }

    return TagData {
        tag_type: tag_type.unwrap(),
        parameters,
        is_end_tag,
    };
}
