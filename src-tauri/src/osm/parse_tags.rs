use crate::{
    common::str_tools::{remove_first, remove_last},
    osm::tag::Tag,
};
use std::collections::HashMap;

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

fn parse_tag(tag: &str) -> Tag {
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

    return Tag {
        tag_type: tag_type.unwrap(),
        parameters,
        is_end_tag,
    };
}

pub fn parse_tags(data: &str) -> Vec<Tag> {
    let tags: Vec<&str> = data.trim().split('>').collect();
    let mut parsed_tags: Vec<Tag> = Vec::with_capacity(tags.len());

    for tag in tags {
        parsed_tags.push(parse_tag(tag));
    }

    return parsed_tags;
}
