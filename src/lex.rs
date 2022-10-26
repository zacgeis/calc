pub enum Token {
    Number(i32),
    Plus,
    Minus,
    Asterisk,
    BackSlash,
    Percent,
    Caret,
}

pub fn tokenize(s: &str) -> Result<Vec<Token>, String> {
    let mut tokens = vec![];
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            ' ' => continue,
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Asterisk),
            '/' => tokens.push(Token::BackSlash),
            '%' => tokens.push(Token::Percent),
            '^' => tokens.push(Token::Caret),
            '0'..='9' => {
                let mut temp = String::new();
                temp.push(c);
                while let Some(k) = chars.peek() {
                    // TODO: fix tihs.
                    temp.push(k);
                }
                match temp.parse() {
                    Ok(num) => tokens.push(Token::Number(num)),
                    _ => return Err("Invalid number literal.".to_string()),
                }
            }
            _ => return Err("Unexpected char found.".to_string()),
        }
    }
    Ok(tokens)
}
