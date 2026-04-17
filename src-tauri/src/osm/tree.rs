use crate::osm::tag::TagData;
use serde::{Deserialize, Serialize};

/// Main Osm data type.
/// Contains no osm data but an index referencing a list of `TagData`, children of `Tag`.
///
/// ## Parameters
///  * `0`: index of data
///  * `1`: children
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tag(pub(crate) usize, pub(crate) TagTree);

impl Tag {
    /// Find child of tag matching predicate
    ///
    /// Returns tag matching predicate if any
    ///
    /// ## Parameters
    ///  * `predicate`: predicate to evaluate from
    ///
    pub fn find_child<P>(&self, predicate: P) -> Option<Box<Tag>>
    where
        P: FnMut(&&Box<Tag>) -> bool,
    {
        let res = self.1.iter().find(predicate);

        if res.is_none() {
            return None;
        }

        return Some(res.unwrap().clone());
    }

    /// Filters children of tag matching predicate
    ///
    /// Returns tags matching predicate
    ///
    /// ## Parameters
    ///  * `predicate`: predicate to evaluate from
    ///
    pub fn filter_children<P>(&self, predicate: P) -> TagTree
    where
        P: FnMut(&&Box<Tag>) -> bool,
    {
        self.1.iter().filter(predicate).cloned().collect()
    }
}

pub type TagTree = Vec<Box<Tag>>;

fn create_tree(tags: &[TagData], mut index: usize, parent_type: &String) -> TagTree {
    let mut return_arr: TagTree = vec![];
    let mut parent: Option<&TagData> = None;
    let mut parent_index: Option<usize> = None;

    loop {
        if tags.len() <= index {
            break;
        }

        let tag = &tags[index];

        if &tag.tag_type == parent_type {
            break;
        };

        index += 1;

        // If theres no children
        if tag.is_end_tag && parent.is_none() {
            return_arr.push(Box::new(Tag(index - 1, vec![])));
            continue;
        }

        // If there is no parent and the tag is not an end tag (checked above)
        if parent.is_none() {
            parent = Some(tag);
            parent_index = Some(index - 1);
            continue;
        }

        // If its not the enclosing end tag
        if !tag.is_end_tag {
            continue;
        }

        // If the tag is not the same as the parent
        if tag.tag_type != parent.unwrap().tag_type {
            continue;
        }

        {
            let parent = parent.unwrap();
            let parent_index = parent_index.unwrap();

            return_arr.push(Box::new(Tag(
                parent_index,
                create_tree(&tags, parent_index + 1, &parent.tag_type),
            )));
        }

        // Reset the parent values
        parent = None;
        parent_index = None;
    }

    if !parent.is_none() {
        let parent_index = parent_index.unwrap();

        return_arr.push(Box::new(Tag(
            parent_index,
            create_tree(&tags, parent_index + 1, &parent.unwrap().tag_type),
        )));
    }

    return return_arr;
}

/// Creates `TagTree` from taglist
pub fn from_tags(tags: &[TagData]) -> TagTree {
    return create_tree(&tags, 0, &"poopcock".to_string());
}
