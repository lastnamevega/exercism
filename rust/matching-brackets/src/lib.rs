pub fn brackets_are_balanced(string: &str) -> bool {
    let mut unterminated_chars: Vec<char> = vec![];

    for c in string.chars() {
        match c {
            '[' | '{' | '(' => unterminated_chars.push(c),
            ']' => {
                if unterminated_chars.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if unterminated_chars.pop() != Some('{') {
                    return false;
                }
            }
            ')' => {
                if unterminated_chars.pop() != Some('(') {
                    return false;
                }
            }
            _ => (),
        }
    }

    unterminated_chars.is_empty()
}
