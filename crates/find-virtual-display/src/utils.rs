pub fn split_once(text: &str, sep: char) -> Option<(&str, &str)> {
    for (current_index, current_char) in text.char_indices() {
        if current_char == sep {
            return Some((&text[..current_index], &text[current_index + 1..]));
        }
    }

    None
}

#[test]
fn test_positive() {
    assert_eq!(split_once("0-255", '-'), Some(("0", "255")));
}

#[test]
fn test_negative() {
    assert_eq!(split_once("0-255", ':'), None);
}
