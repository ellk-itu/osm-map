use crate::osm::tag::TagTree;

fn flatten_tags(data: &TagTree) -> TagTree {
    let mut flat_vec: TagTree = vec![];

    for tag in data {
        flat_vec.push(tag.clone());

        if !tag.children.is_none() {
            let children = tag.children.as_ref().unwrap();
            flatten_tags(children)
                .iter()
                .for_each(|tag| flat_vec.push(tag.clone()));
        }
    }
    return flat_vec;
}

pub fn flatten(data: &TagTree) -> TagTree {
    let mut flat_vec: TagTree = vec![];

    for tag in data {
        flat_vec.push(tag.clone());

        if !tag.children.is_none() {
            let children = tag.children.as_ref().unwrap();
            flatten_tags(children)
                .iter()
                .for_each(|tag| flat_vec.push(tag.clone()));
        }
    }

    return flat_vec;
}
