use split_first_char::split_first_char;

const fn is_valid_status(char: char) -> bool {
    matches!(char, '?' | 'A' | 'D' | 'M' | ' ')
}

#[derive(Debug)]
pub struct Value<'a> {
    pub plain: &'a str,
    pub staged: char,
    pub unstaged: char,
    pub path: &'a str,
}

impl<'a> Value<'a> {
    pub fn parse_single_line(line: &'a str) -> Option<Self> {
        let plain = line;

        let (staged, line) = split_first_char(line)?;
        if !is_valid_status(staged) {
            return None;
        }

        let (unstaged, line) = split_first_char(line)?;
        if !is_valid_status(unstaged) {
            return None;
        }

        let (' ', path) = split_first_char(line)? else {
            return None;
        };

        Some(Value {
            plain,
            staged,
            unstaged,
            path,
        })
    }
}
