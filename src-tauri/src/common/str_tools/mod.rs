use std::ops::Range;

pub fn remove_from(string: &str, range: Range<usize>) -> &str {
    return if string.len() > range.len() {
        &string[range]
    } else {
        &string
    };
}

pub fn remove_first(string: &str) -> &str {
    return if string.len() > 1 {
        &string[1..]
    } else {
        &string
    };
}

pub fn remove_last(string: &str) -> &str {
    return if string.len() > 1 {
        &string[0..string.len() - 1]
    } else {
        &string
    };
}
