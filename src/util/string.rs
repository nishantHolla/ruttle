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
    string.get(start..)?.find('\n').map(|i| start + i)
}

pub fn split_quoted(input: &str) -> impl Iterator<Item = String> + '_ {
    struct SplitQuoted<'a> {
        input: &'a str,
        pos: usize,
    }

    impl<'a> Iterator for SplitQuoted<'a> {
        type Item = String;

        fn next(&mut self) -> Option<Self::Item> {
            let bytes = self.input.as_bytes();
            let len = bytes.len();

            // Skip leading whitespace
            while self.pos < len && bytes[self.pos].is_ascii_whitespace() {
                self.pos += 1;
            }

            if self.pos >= len {
                return None;
            }

            let mut out = String::new();
            let mut in_quotes = false;

            while self.pos < len {
                let c = bytes[self.pos] as char;

                match c {
                    '"' => {
                        in_quotes = !in_quotes;
                        out.push(c)
                    }
                    c if c.is_whitespace() && !in_quotes => {
                        break;
                    }
                    _ => out.push(c),
                }

                self.pos += 1;
            }

            Some(out)
        }
    }

    SplitQuoted { input, pos: 0 }
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

pub fn normalize_whitespace(input: &str, limit: Option<usize>) -> String {
    let mut result = String::with_capacity(input.len());
    let mut in_quotes = false;
    let mut prev_was_whitespace = false;
    let mut count: usize = 0;
    let mut done = false;

    for c in input.chars() {
        match c {
            '"' => {
                if done {
                    result.push(c);
                    continue;
                }

                in_quotes = !in_quotes;
                result.push(c);
                if prev_was_whitespace {
                    count += 1;
                    if let Some(target) = limit
                        && count >= target
                    {
                        done = true;
                    }
                }
                prev_was_whitespace = false;
            }

            c if c.is_whitespace() && !in_quotes => {
                if done {
                    result.push(c);
                    continue;
                }

                if !prev_was_whitespace {
                    result.push(' ');
                    prev_was_whitespace = true;
                }
            }

            _ => {
                if done {
                    result.push(c);
                    continue;
                }

                result.push(c);
                if prev_was_whitespace {
                    count += 1;
                    if let Some(target) = limit
                        && count >= target
                    {
                        done = true;
                    }
                }
                prev_was_whitespace = false;
            }
        }
    }

    result
}
