use std::{collections::HashMap, hash::Hash};

use anyhow::Result;

use crate::{errors::JsonError, JsonValue, Token, TokenType};

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

    pub fn parse(&mut self) -> Result<JsonValue> {
        let tk = self.advance();
        match tk {
            Some(&Token::ObjectStart) => Ok(self.parse_objct()?),
            Some(&Token::ArrayStart) => Ok(self.parse_array()?),
            _ => Err(JsonError::InvalidJson)?,
        }
    }

    // object: {}
    fn parse_objct(&mut self) -> Result<JsonValue> {
        let mut result: HashMap<String, JsonValue> = HashMap::new();

        loop {
            if self.tokens[self.current] == Token::ObjectEnd {
                break;
            }

            let key = self.consume_key()?;
            self.consume(TokenType::Colon)?;

            let value = self.parse_value()?;
            result.insert(key, value);
        }

        Ok(JsonValue::Object(result))
    }

    fn parse_array(&mut self) -> Result<JsonValue> {
        todo!()
    }

    fn parse_value(&mut self) -> Result<JsonValue> {
        let tk = self.advance();
        if tk.is_none() {
            return Err(JsonError::InvalidJson)?;
        }

        let cur_tk = tk.unwrap();

        match cur_tk {
            Token::ObjectStart => Ok(self.parse_objct()?),
            Token::ArrayStart => Ok(self.parse_array()?),
            Token::String(v) => Ok(JsonValue::SString(v.clone())),
            Token::Number(v) => Ok(JsonValue::Number(*v)),
            Token::Bool(v) => Ok(JsonValue::Boolean(*v)),
            Token::Null => Ok(JsonValue::Null),
            _ => Err(JsonError::InvalidJson)?,
        }
    }

    fn consume_key(&mut self) -> Result<String> {
        if let Token::String(v) = self.consume(TokenType::String)? {
            return Ok(v);
        }

        Err(JsonError::ExpectedString)?
    }

    fn consume(&mut self, tk_type: TokenType) -> Result<Token> {
        if let Some(v) = self.advance() {
            match tk_type {
                TokenType::String => {
                    if let Token::String(v) = v {
                        Ok(Token::String(v.clone()))
                    } else {
                        Err(JsonError::ExpectToken("String".to_string(), v.to_string()))?
                    }
                }
                TokenType::Number => {
                    if let Token::Number(v) = *v {
                        Ok(Token::Number(v))
                    } else {
                        Err(JsonError::ExpectToken("Number".to_string(), v.to_string()))?
                    }
                }
                TokenType::ObjectStart => {
                    if Token::ObjectStart == *v {
                        Ok(Token::ObjectStart)
                    } else {
                        Err(JsonError::InvalidJson)?
                    }
                }
                TokenType::ObjectEnd => {
                    if Token::ObjectEnd == *v {
                        Ok(Token::ObjectEnd)
                    } else {
                        Err(JsonError::InvalidJson)?
                    }
                }
                TokenType::Colon => {
                    if Token::Colon == *v {
                        Ok(Token::Colon)
                    } else {
                        Err(JsonError::InvalidJson)?
                    }
                }
                TokenType::Comma => {
                    if Token::Comma == *v {
                        Ok(Token::Comma)
                    } else {
                        Err(JsonError::InvalidJson)?
                    }
                }
                _ => Err(JsonError::InvalidJson)?,
            }
        } else {
            Err(JsonError::InvalidJson)?
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
