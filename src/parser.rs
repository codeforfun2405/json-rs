use std::collections::HashMap;

use crate::{errors::JsonError, JsonValue, Token, TokenType};
use anyhow::Result;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            current: 0,
        }
    }

    // parse tokens into JsonValue
    // there are 2 cases
    // first: the json is a json object: {}
    // second: the json is an array: []
    pub fn parse(&mut self) -> Result<JsonValue> {
        if let Some(cur_tk) = self.advance() {
            match cur_tk {
                Token::ObjectStart => Ok(self.parse_object()?),
                Token::ArrayStart => Ok(self.parse_array()?),
                _ => Err(JsonError::InvalidJson)?,
            }
        } else {
            Err(JsonError::InvalidJson)?
        }
    }

    // object: {"key": JsonValue, "key1": JsonValue}
    fn parse_object(&mut self) -> Result<JsonValue> {
        // parse every key-value parir utill meet ObjectEnd: }
        let mut object = HashMap::new();

        while let Some(tk) = self.peek() {
            if tk == &Token::ObjectEnd {
                break;
            }

            let key = self.consume(TokenType::String)?; // consume key
            self.consume(TokenType::Colon)?; // consume :

            let value = self.parse_value()?; // parse json value
            object.insert(Self::token_to_string(key)?, value);

            // consume comma between key-value pair if not meet ObjectEnd
            if let Some(v) = self.peek() {
                if v != &Token::ObjectEnd {
                    self.consume(TokenType::Comma)?;
                }
            }
        }

        self.consume(TokenType::ObjectEnd)?;
        Ok(JsonValue::Object(object))
    }

    fn token_to_string(tk: Token) -> Result<String> {
        match tk {
            Token::String(v) => Ok(v.clone()),
            _ => Err(JsonError::ExpectedString)?,
        }
    }

    // parse array: [JsonValue, JsonValue]
    fn parse_array(&mut self) -> Result<JsonValue> {
        let mut array = Vec::new();

        while let Some(tk) = self.peek() {
            if tk == &Token::ArrayEnd {
                break;
            }

            let value = self.parse_value()?;
            array.push(value);

            // consume comma between elements if not meet ArrayEnd
            if let Some(v) = self.peek() {
                if v != &Token::ArrayEnd {
                    self.consume(TokenType::Comma)?;
                }
            }
        }

        self.consume(TokenType::ArrayEnd)?;
        Ok(JsonValue::Array(array))
    }

    // parse token into JsonValue
    // if current token is { => parse object
    // if current token is [ => parse array
    // if current token is String => return JsonValue::JString
    // if current token is Number => return JsonValue::Number
    // if current token is Bool => return JsonValue::Boolean
    // if current token is null => return JsonValue::Null
    fn parse_value(&mut self) -> Result<JsonValue> {
        if let Some(tk) = self.advance() {
            match tk {
                Token::String(v) => Ok(JsonValue::JString(v.clone())),
                Token::Number(v) => Ok(JsonValue::Number(*v)),
                Token::Bool(v) => Ok(JsonValue::Boolean(*v)),
                Token::Null => Ok(JsonValue::Null),
                Token::ObjectStart => Ok(self.parse_object()?),
                Token::ArrayStart => Ok(self.parse_array()?),
                _ => Err(JsonError::InvalidJson)?,
            }
        } else {
            Err(JsonError::InvalidJson)?
        }
    }

    // check if the current token is same token type with `tk_type`
    // advance the current index and return the token in current index
    fn consume(&mut self, tk_type: TokenType) -> Result<Token> {
        if let Some(cur_tk) = self.advance() {
            match tk_type {
                TokenType::String => Self::consume_string(cur_tk),
                TokenType::Number => Self::consume_number(cur_tk),
                TokenType::ObjectEnd => Self::consume_object_end(cur_tk),
                TokenType::ArrayEnd => Self::consume_array_end(cur_tk),
                TokenType::Colon => Self::consume_colon(cur_tk),
                TokenType::Comma => Self::consume_comma(cur_tk),
                _ => Err(JsonError::InvalidJson)?,
            }
        } else {
            Err(JsonError::InvalidJson)?
        }
    }

    fn consume_string(cur_tk: &Token) -> Result<Token> {
        if let Token::String(_) = cur_tk {
            Ok(cur_tk.clone())
        } else {
            Err(JsonError::ExpectToken(
                "String".to_string(),
                cur_tk.to_string(),
            ))?
        }
    }

    fn consume_number(cur_tk: &Token) -> Result<Token> {
        if let Token::Number(_) = cur_tk {
            Ok(cur_tk.clone())
        } else {
            Err(JsonError::ExpectToken(
                "Number".to_string(),
                cur_tk.to_string(),
            ))?
        }
    }

    fn consume_object_end(cur_tk: &Token) -> Result<Token> {
        if let Token::ObjectEnd = cur_tk {
            Ok(cur_tk.clone())
        } else {
            Err(JsonError::ExpectToken("}".to_string(), cur_tk.to_string()))?
        }
    }

    fn consume_array_end(cur_tk: &Token) -> Result<Token> {
        if let Token::ArrayEnd = cur_tk {
            Ok(cur_tk.clone())
        } else {
            Err(JsonError::ExpectToken("]".to_string(), cur_tk.to_string()))?
        }
    }

    fn consume_colon(cur_tk: &Token) -> Result<Token> {
        if let Token::Colon = cur_tk {
            Ok(cur_tk.clone())
        } else {
            Err(JsonError::ExpectToken(":".to_string(), cur_tk.to_string()))?
        }
    }

    fn consume_comma(cur_tk: &Token) -> Result<Token> {
        if let Token::Comma = cur_tk {
            Ok(cur_tk.clone())
        } else {
            Err(JsonError::ExpectToken(",".to_string(), cur_tk.to_string()))?
        }
    }

    fn is_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn peek(&self) -> Option<&Token> {
        if self.is_end() {
            return None;
        }

        Some(&self.tokens[self.current])
    }

    fn advance(&mut self) -> Option<&Token> {
        if self.is_end() {
            return None;
        }

        self.current += 1;
        Some(&self.tokens[self.current - 1])
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::scanner;

    use super::*;

    #[test]
    fn test_parse_object() -> anyhow::Result<()> {
        let json = r#"{"name": "Alex", "age": 28, "Loc": {"city":"New York"}}"#;
        let mut json_scanner = scanner::Scanner::new(json.to_string());
        let tokens = json_scanner.scan()?;

        let mut parser = Parser::new(tokens);
        let json_value = parser.parse()?;

        let mut expect_object = HashMap::new();
        expect_object.insert("name".to_string(), JsonValue::JString("Alex".to_string()));
        expect_object.insert("age".to_string(), JsonValue::Number(28 as f64));
        expect_object.insert(
            "Loc".to_string(),
            JsonValue::Object(HashMap::from_iter([(
                "city".to_string(),
                JsonValue::JString("New York".to_string()),
            )])),
        );

        let expect_json_value = JsonValue::Object(expect_object);
        assert_eq!(expect_json_value, json_value);
        Ok(())
    }

    #[test]
    fn test_parse_simple_array() -> anyhow::Result<()> {
        let json = r#"[1, 2, 3, "A", "Hello", "Good Night"]"#;
        let mut json_scanner = scanner::Scanner::new(json.to_string());
        let tokens = json_scanner.scan()?;

        println!("tokens: {:?}", tokens);

        let mut parser = Parser::new(tokens);
        let json_value = parser.parse()?;
        assert_eq!(
            JsonValue::Array(vec![
                JsonValue::Number(1 as f64),
                JsonValue::Number(2 as f64),
                JsonValue::Number(3 as f64),
                JsonValue::JString("A".to_string()),
                JsonValue::JString("Hello".to_string()),
                JsonValue::JString("Good Night".to_string()),
            ]),
            json_value
        );
        Ok(())
    }

    #[test]
    fn test_parse_simple_object() -> anyhow::Result<()> {
        let json = r#"{"key1": "Test1", "key2": "Awesome"}"#;
        let mut json_scanner = scanner::Scanner::new(json.to_string());
        let tokens = json_scanner.scan()?;

        println!("tokens: {:?}", tokens);

        let mut parser = Parser::new(tokens);
        let json_value = parser.parse()?;
        assert_eq!(
            JsonValue::Object(HashMap::from_iter([
                ("key1".to_string(), JsonValue::JString("Test1".to_string())),
                (
                    "key2".to_string(),
                    JsonValue::JString("Awesome".to_string())
                )
            ])),
            json_value
        );
        Ok(())
    }

    #[test]
    fn test_parse_array() -> anyhow::Result<()> {
        let json = r#"{"city_list":[{"name":"A", "population": 1000}, {"name":"B", "population": 30000}]}"#;
        let mut json_scanner = scanner::Scanner::new(json.to_string());
        let tokens = json_scanner.scan()?;

        println!("tokens: {:?}", tokens);

        let mut parser = Parser::new(tokens);
        let json_value = parser.parse()?;

        let mut expect_object = HashMap::new();
        expect_object.insert(
            "city_list".to_string(),
            JsonValue::Array(vec![
                JsonValue::Object(HashMap::from_iter([
                    ("name".to_string(), JsonValue::JString("A".to_string())),
                    ("population".to_string(), JsonValue::Number(1000 as f64)),
                ])),
                JsonValue::Object(HashMap::from_iter([
                    ("name".to_string(), JsonValue::JString("B".to_string())),
                    ("population".to_string(), JsonValue::Number(30000 as f64)),
                ])),
            ]),
        );

        let expect_json_value = JsonValue::Object(expect_object);
        assert_eq!(expect_json_value, json_value);
        Ok(())
    }
}
