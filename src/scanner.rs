use anyhow::Result;

use crate::{errors::JsonError, Token};

#[derive(Debug)]
pub struct Scanner {
    chars: Vec<char>,
    start: usize,
    current: usize,
}

impl Scanner {
    pub fn new(input: String) -> Self {
        Self {
            chars: input.chars().collect(),
            start: 0,
            current: 0,
        }
    }

    pub fn scan(&mut self) -> Result<Vec<Token>> {
        let mut tokens: Vec<Token> = vec![];

        loop {
            if self.is_end() {
                break;
            }

            self.start = self.current; // make start catchup current char index
            let tk = self.scan_token()?;
            if tk == Token::WhiteSpace {
                continue;
            }

            tokens.push(tk);
        }

        Ok(tokens)
    }

    fn scan_token(&mut self) -> Result<Token> {
        let ch = self.advance();
        if ch.is_none() {
            return Err(JsonError::InvalidJson)?;
        }

        let cur_ch = ch.unwrap();

        let res = match cur_ch {
            '{' => Ok(Token::ObjectStart),
            '}' => Ok(Token::ObjectEnd),
            '[' => Ok(Token::ArrayStart),
            ']' => Ok(Token::ArrayEnd),
            ',' => Ok(Token::Comma),
            ':' => Ok(Token::Colon),
            '"' => self.scan_string(),
            ' ' | '\t' | '\n' | '\r' => Ok(Token::WhiteSpace),
            _ => {
                if cur_ch.is_alphabetic() {
                    return self.scan_ident();
                }
                if cur_ch.is_numeric() {
                    return self.scan_number();
                }

                Err(JsonError::UnsupportedChar(cur_ch))?
            }
        };

        res
    }

    fn scan_string(&mut self) -> Result<Token> {
        while let Some(ch) = self.peek() {
            if ch == '"' || self.is_end() {
                break;
            }
            self.advance();
        }

        if self.is_end() {
            return Err(JsonError::UnterminatedString)?;
        }

        self.advance(); // skip right '"'

        let s = String::from_iter(&self.chars[self.start + 1..self.current - 1]);
        Ok(Token::String(s))
    }

    fn scan_number(&mut self) -> Result<Token> {
        self.peek_number();

        let num_str = String::from_iter(&self.chars[self.start..self.current]);
        let num = num_str.parse::<f64>()?;
        Ok(Token::Number(num))
    }

    fn peek_number(&mut self) {
        while let Some(v) = self.peek() {
            if !v.is_numeric() {
                break;
            }
            self.advance();
        }

        let dot_opt = self.peek();
        if dot_opt.is_none() || dot_opt.unwrap() != '.' {
            return;
        }

        if let Some(v) = self.peek_next() {
            if !v.is_numeric() {
                return;
            }

            self.advance();
            while let Some(n) = self.peek() {
                if !n.is_numeric() {
                    break;
                }
                self.advance();
            }
        }
    }

    fn scan_ident(&mut self) -> Result<Token> {
        while let Some(ch) = self.peek() {
            if ch.is_alphanumeric() {
                self.advance();
            } else {
                break;
            }
        }

        if self.is_end() {
            return Err(JsonError::InvalidJson)?;
        }

        let s = String::from_iter(&self.chars[self.start..self.current]);
        match s.as_str() {
            "true" => Ok(Token::Bool(true)),
            "false" => Ok(Token::Bool(false)),
            "null" => Ok(Token::Null),
            _ => Err(JsonError::UnknowIdent(s))?,
        }
    }

    fn advance(&mut self) -> Option<char> {
        if self.is_end() {
            return None;
        }

        self.current += 1;
        Some(self.chars[self.current - 1])
    }

    fn peek(&mut self) -> Option<char> {
        if self.is_end() {
            return None;
        }

        Some(self.chars[self.current])
    }

    fn peek_next(&self) -> Option<char> {
        if self.is_end() {
            return None;
        }
        Some(self.chars[self.current + 1])
    }

    fn is_end(&self) -> bool {
        self.current >= self.chars.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_scan_bracket_and_object() -> Result<()> {
        let json = r#"{[]}"#;
        let mut scanner = Scanner::new(json.to_string());
        let tokens = scanner.scan()?;
        println!("scaned tokens: {:?}", tokens);
        assert_eq!(
            vec![
                Token::ObjectStart,
                Token::ArrayStart,
                Token::ArrayEnd,
                Token::ObjectEnd,
            ],
            tokens
        );
        Ok(())
    }

    #[test]
    fn test_scan_string() -> Result<()> {
        let json = r#"{"key":"value"}"#;
        let mut scanner = Scanner::new(json.to_string());
        let tokens = scanner.scan()?;
        println!("scaned tokens: {:?}", tokens);
        assert_eq!(
            vec![
                Token::ObjectStart,
                Token::String("key".to_string()),
                Token::Colon,
                Token::String("value".to_string()),
                Token::ObjectEnd,
            ],
            tokens
        );
        Ok(())
    }

    #[test]
    fn test_scan_number() -> Result<()> {
        let json = r#"{"key":99.999}"#;
        let mut scanner = Scanner::new(json.to_string());
        let tokens = scanner.scan()?;
        println!("scaned tokens: {:?}", tokens);
        assert_eq!(
            vec![
                Token::ObjectStart,
                Token::String("key".to_string()),
                Token::Colon,
                Token::Number(99.999 as f64),
                Token::ObjectEnd,
            ],
            tokens
        );
        Ok(())
    }

    #[test]
    fn test_scan_identifier() -> Result<()> {
        let json = r#"{"is_ok": true, "is_not_ok": false, "exists": null}"#;
        let mut scanner = Scanner::new(json.to_string());
        let tokens = scanner.scan()?;
        // scaned tokens: [ObjectStart, String("is_ok"), Colon, Bool(true), Comma, String("is_not_ok"), Colon, Bool(false), Comma, String("exists"), Colon, Null, ObjectEnd]
        println!("scaned tokens: {:?}", tokens);
        assert_eq!(
            vec![
                Token::ObjectStart,
                Token::String("is_ok".to_string()),
                Token::Colon,
                Token::Bool(true),
                Token::Comma,
                Token::String("is_not_ok".to_string()),
                Token::Colon,
                Token::Bool(false),
                Token::Comma,
                Token::String("exists".to_string()),
                Token::Colon,
                Token::Null,
                Token::ObjectEnd,
            ],
            tokens
        );
        Ok(())
    }
}
