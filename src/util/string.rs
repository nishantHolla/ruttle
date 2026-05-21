pub fn find_directive_end(text: &str, start: usize) -> Option<usize> {
    if text.get(start..=start)? != "{" {
        return None;
    }

    let mut depth = 1;
    let mut in_quotes = false;
    let mut escaped = false;

    for (offset, ch) in text[start + 1..].char_indices() {
        let idx = start + 1 + offset;

        match ch {
            '\\' if in_quotes => {
                escaped = !escaped;
            }

            '"' if !escaped => {
                in_quotes = !in_quotes;
                escaped = false;
            }

            '{' if !in_quotes => {
                depth += 1;
                escaped = false;
            }

            '}' if !in_quotes => {
                depth -= 1;
                escaped = false;

                if depth == 0 {
                    return Some(idx);
                }
            }

            _ => {
                escaped = false;
            }
        }
    }

    None
}

pub fn get_row_col(string: &str, index: usize) -> Option<(usize, usize)> {
    if index > string.len() {
        return None;
    }

    let mut row = 1;
    let mut col = 1;

    for (i, ch) in string.char_indices() {
        if i >= index {
            break;
        }

        if ch == '\n' {
            row += 1;
            col = 1;
        } else {
            col += 1;
        }
    }

    Some((row, col))
}

pub fn get_substr(string: &str, start: usize, end: usize) -> Option<String> {
    if start >= string.len() || end >= string.len() {
        None
    } else {
        Some(string[start..=end].to_string())
    }
}

pub fn get_line_end_index(string: &str, start: usize) -> Option<usize> {
    if start >= string.len() {
        None
    } else {
        let slice = &string[start..];
        if let Some(end) = slice.find('\n') {
            Some(start + end)
        } else {
            Some(string.len() - 1)
        }
    }
}

pub fn indent_with_pipes(string: &str) -> String {
    let indent_str = String::from("| ");
    let mut result = indent_str.clone();

    for (i, c) in string.chars().enumerate() {
        result.push(c);
        if c == '\n' && i != string.len() - 1 {
            result.push_str(&indent_str);
        }
    }

    result
}
