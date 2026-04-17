use std::ops::Range;

/// Removes string slice in range
pub fn remove_from(string: &str, range: Range<usize>) -> &str {
    return if string.len() > range.len() {
        &string[range]
    } else {
        &string
    };
}

/// Removes first character in str slice
/// Returns unmodified str slice if `length < 1`
pub fn remove_first(string: &str) -> &str {
    return if string.len() < 1 {
        &string
    } else {
        &string[1..]
    };
}

/// Removes last character in str slice
/// Returns unmodified str slice if `length < 1`
pub fn remove_last(string: &str) -> &str {
    return if string.len() < 1 {
        &string
    } else {
        &string[0..string.len() - 1]
    };
}
