#[derive(Debug, Eq, PartialEq)]
pub enum Token {
    Num(i32),
    OpenParen,
    CloseParen,
    Plus,
    Minus,
    Asterisk,
    BackSlash,
    Percent,
    Caret,
}

pub fn tokenize(s: &str) -> Result<Vec<Token>, String> {
    let mut tokens = vec![];
    let mut chars = s.char_indices().peekable();
    while let Some((i, c)) = chars.next() {
        match c {
            ' ' => continue,
            '(' => tokens.push(Token::OpenParen),
            ')' => tokens.push(Token::CloseParen),
            '+' => tokens.push(Token::Plus),
            '-' => tokens.push(Token::Minus),
            '*' => tokens.push(Token::Asterisk),
            '/' => tokens.push(Token::BackSlash),
            '%' => tokens.push(Token::Percent),
            '^' => tokens.push(Token::Caret),
            '0'..='9' => {
                let mut end = s.len();
                while let Some((k, c)) = chars.peek() {
                    match c {
                        '0'..='9' => {
                            chars.next();
                        }
                        _ => {
                            end = *k;
                            break;
                        }
                    }
                }
                match s[i..end].parse() {
                    Ok(num) => tokens.push(Token::Num(num)),
                    _ => return Err("Invalid number literal.".to_string()),
                }
            }
            _ => return Err(format!("Unexpected char found: {}.", c)),
        }
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        assert_eq!(tokenize("#"), Err("Unexpected char found: #.".to_string()));

        assert_eq!(tokenize("1234"), Ok(vec![Token::Num(1234)]));

        assert_eq!(
            tokenize("-1234"),
            Ok(vec![Token::Minus, Token::Num(1234)])
        );

        assert_eq!(
            tokenize("- 1234"),
            Ok(vec![Token::Minus, Token::Num(1234)])
        );

        assert_eq!(
            tokenize("12 34"),
            Ok(vec![Token::Num(12), Token::Num(34)])
        );

        assert_eq!(
            tokenize("1 + 1"),
            Ok(vec![Token::Num(1), Token::Plus, Token::Num(1)])
        );

        assert_eq!(
            tokenize("12 + 34 - 56 * 78"),
            Ok(vec![
                Token::Num(12),
                Token::Plus,
                Token::Num(34),
                Token::Minus,
                Token::Num(56),
                Token::Asterisk,
                Token::Num(78),
            ])
        );
    }
}
