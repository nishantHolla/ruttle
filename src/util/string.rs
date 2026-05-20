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
