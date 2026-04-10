use crate::osm::tag::{Tag, TagTree, TagWithChildren};

#[derive(Debug)]
pub struct Node(pub(crate) usize, pub(crate) Vec<Box<Node>>);

fn create_reference_tree(tags: &Vec<Tag>, start_index: Option<usize>) -> Vec<Box<Node>> {
    let mut i = start_index.unwrap_or(0);
    let mut iter = tags.iter();
    let mut return_arr: Vec<Box<Node>> = vec![];
    let mut first = true;

    let mut parent: Option<&Tag> = None;
    let mut parent_index: Option<usize> = None;
    let mut children: Vec<Tag> = vec![];

    loop {
        let next = iter.next();

        if !first {
            i += 1;
        } else {
            first = false
        }

        // Break when there is no more tags
        if next.is_none() {
            break;
        }

        let tag = next.unwrap();

        // If its a tag that closes itself
        if tag.is_end_tag && parent.is_none() {
            return_arr.push(Box::new(Node(i, Vec::new())));
            continue;
        }

        if !parent.is_none() {
            if parent.unwrap().tag_type == tag.tag_type {
                return_arr.push(Box::new(Node(
                    parent_index.unwrap(),
                    create_reference_tree(&children.clone(), Some(parent_index.unwrap() + 1)),
                )));
                parent = None;
                children = vec![];
                continue;
            }

            children.push(tag.clone());
            continue;
        }

        parent = Some(tag);
        parent_index = Some(i);
    }

    return return_arr;
}

fn create_tag_tree(tags: &Vec<Tag>, nodes: Vec<Box<Node>>) -> TagTree {
    let mut result_arr: TagTree = vec![];

    for node in nodes {
        let tag = tags[node.0].clone();

        if node.1.len() <= 0 {
            result_arr.push(Box::new(TagWithChildren {
                tag_type: tag.tag_type,
                parameters: tag.parameters,
                children: None,
            }));
            continue;
        }

        result_arr.push(Box::new(TagWithChildren {
            tag_type: tag.tag_type,
            parameters: tag.parameters,
            children: Some(create_tag_tree(&tags, node.1)),
        }));
    }

    return result_arr;
}

pub fn create_tree(parsed_tags: Vec<Tag>) -> TagTree {
    let reference_tree = create_reference_tree(&parsed_tags, None);
    let tag_tree = create_tag_tree(&parsed_tags, reference_tree);
    return tag_tree;
}
