use serde::ser::SerializeStruct;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Tag {
    tag_type: String,
    parameters: HashMap<String, String>,
    children: Option<Vec<Box<Tag>>>,
    is_end_tag: bool,
}

impl serde::Serialize for Tag {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Tag", 2)?;
        s.serialize_field("tag-type", &self.tag_type)?;
        s.serialize_field("parameters", &self.parameters)?;
        s.serialize_field("children", &self.children)?;
        s.serialize_field("is-end-tag", &self.is_end_tag)?;

        return s.end();
    }
}

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

fn trim_first_char(string: &str) -> &str {
    let mut chars = string.chars();
    chars.next();

    return chars.as_str();
}

fn is_end_tag(string: &str) -> bool {
    let mut copy = string.chars();
    let first_char = copy.next();
    let last_char = copy.next_back();

    if first_char == Some('/') {
        return true;
    }

    if last_char == Some('/') {
        return true;
    }

    return false;
}

fn parse_tag(tag: &str) -> Tag {
    let trimmed_tag = trim_first_char(tag.trim());
    let split_tag = custom_split(trimmed_tag);

    let is_end_tag = is_end_tag(trimmed_tag);
    let mut tag_type: Option<String> = None;
    let mut parameters: HashMap<String, String> = HashMap::with_capacity(split_tag.len() - 1);

    for (i, param) in split_tag.iter().enumerate() {
        if i == 0 {
            tag_type = Some(param.clone());
            continue;
        }

        let key_value: Vec<&str> = param.split("=").collect();
        if key_value.len() <= 1 {
            println!("{:?}", key_value);
        }
        parameters.insert(key_value[0].to_string(), key_value[1].to_string());
    }

    return Tag {
        tag_type: tag_type.unwrap(),
        parameters,
        children: None,
        is_end_tag,
    };
}

fn create_tree(tags: Vec<Tag>) -> Vec<Box<Tag>> {
    if tags.len() <= 2 {
        return tags.iter().map(|tag| Box::new(tag.clone())).collect();
    }

    let mut tag_tree: Vec<Box<Tag>> = vec![];
    let mut tag_type: String = String::new();
    let mut opening_tag: Option<Box<Tag>> = None;
    let mut children: Vec<Tag> = vec![];

    for tag in tags {
        if opening_tag.as_ref().is_none() {
            tag_type = tag.tag_type.clone();
            opening_tag = Some(Box::new(tag));
            continue;
        }

        if tag.is_end_tag && tag.tag_type == tag_type {
            if let Some(mut open) = opening_tag.take() {
                open.children = Some(create_tree(children));
                tag_tree.push(open);
            }
            children = vec![];
            continue;
        }

        children.push(tag);
    }

    return tag_tree;
}

pub fn parse(data: &str) -> Vec<Tag> {
    let tags: Vec<&str> = data.split('>').collect();
    let mut parsed_tags: Vec<Tag> = Vec::with_capacity(tags.len());

    for tag in tags {
        parsed_tags.push(parse_tag(tag));
    }

    return parsed_tags;
}
