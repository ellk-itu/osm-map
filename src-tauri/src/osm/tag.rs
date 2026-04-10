use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Tag {
    pub(crate) tag_type: String,
    pub(crate) parameters: HashMap<String, String>,
    pub(crate) is_end_tag: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TagWithChildren {
    pub(crate) tag_type: String,
    pub(crate) parameters: HashMap<String, String>,
    pub(crate) children: Option<Vec<Box<TagWithChildren>>>,
}

pub type TagTree = Vec<Box<TagWithChildren>>;

impl TagWithChildren {
    pub fn find<P>(&self, predicate: P) -> Option<Box<TagWithChildren>>
    where
        P: FnMut(&&Box<TagWithChildren>) -> bool,
    {
        self.children
            .as_ref()
            .unwrap()
            .iter()
            .find(predicate)
            .cloned()
    }

    pub fn find_all<P>(&self, predicate: P) -> TagTree
    where
        P: FnMut(&&Box<TagWithChildren>) -> bool,
    {
        self.children
            .as_ref()
            .unwrap()
            .iter()
            .filter(predicate)
            .cloned()
            .collect()
    }
}
