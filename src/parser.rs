use crate::{errors::JsonError, JsonValue, Token, TokenType};
use anyhow::Result;
use std::collections::HashMap;

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
            if self.is_end() || self.tokens[self.current] == Token::ObjectEnd {
                break;
            }

            let key = self.consume_key()?;
            println!("[parse_objct] consume key: {}", key);
            self.consume(TokenType::Colon)?;

            let value = self.parse_value()?;
            println!("[parse_objct] parsed value: {:?}", value);

            result.insert(key, value);

            if let Some(v) = self.peek() {
                if v != &Token::ObjectEnd {
                    self.consume(TokenType::Comma)?;
                }
            }
        }

        self.consume(TokenType::ObjectEnd)?;

        Ok(JsonValue::Object(result))
    }

    fn parse_array(&mut self) -> Result<JsonValue> {
        println!("[parse_array] start parse array, current: {}", self.current);

        let mut arr: Vec<JsonValue> = Vec::new();
        while let Some(v) = self.peek() {
            if v == &Token::ArrayEnd {
                break;
            }

            println!(
                "[parse_array] start parse_value: {}",
                self.tokens[self.current]
            );

            let arr_element = self.parse_value()?;
            println!("[parse_array] add {:?} to array", arr_element);

            arr.push(arr_element);

            println!("[parse_array] next token: {:?}", self.peek());

            // if the array is not end, need consume the comma between elements
            if let Some(t) = self.peek() {
                if t != &Token::ArrayEnd {
                    self.consume(TokenType::Comma)?;
                }
            }
        }

        println!("consume array end...");
        self.consume(TokenType::ArrayEnd)?;
        Ok(JsonValue::Array(arr))
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
                        Err(JsonError::ExpectToken("{".to_string(), v.to_string()))?
                    }
                }
                TokenType::ObjectEnd => {
                    if Token::ObjectEnd == *v {
                        Ok(Token::ObjectEnd)
                    } else {
                        Err(JsonError::ExpectToken("}".to_string(), v.to_string()))?
                    }
                }
                TokenType::ArrayEnd => {
                    if Token::ArrayEnd == *v {
                        Ok(Token::ArrayEnd)
                    } else {
                        Err(JsonError::ExpectToken("]".to_string(), v.to_string()))?
                    }
                }
                TokenType::Colon => {
                    if Token::Colon == *v {
                        Ok(Token::Colon)
                    } else {
                        Err(JsonError::ExpectToken(":".to_string(), v.to_string()))?
                    }
                }
                TokenType::Comma => {
                    if Token::Comma == *v {
                        Ok(Token::Comma)
                    } else {
                        Err(JsonError::ExpectToken(",".to_string(), v.to_string()))?
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

#[cfg(test)]
mod tests {
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
        expect_object.insert("name".to_string(), JsonValue::SString("Alex".to_string()));
        expect_object.insert("age".to_string(), JsonValue::Number(28 as f64));
        expect_object.insert(
            "Loc".to_string(),
            JsonValue::Object(HashMap::from_iter([(
                "city".to_string(),
                JsonValue::SString("New York".to_string()),
            )])),
        );

        let expect_json_value = JsonValue::Object(expect_object);
        assert_eq!(expect_json_value, json_value);
        Ok(())
    }

    #[test]
    fn test_parse_array() -> anyhow::Result<()> {
        /*
        tokens: [ObjectStart, String("city_list"), Colon, ArrayStart, ObjectStart, String("name"), Colon, String("A"), Comma, String("population"), Colon, Number(1000.0), ObjectEnd, Comma, ObjectStart, String("name"), Colon, String("B"), Comma, String("population"), Colon, Number(30000.0), ObjectEnd, ArrayEnd, ObjectEnd]
        [parse_objct] consume key: city_list
        [parse_array] start parse array, current: 4
        [parse_array] start parse_value: {
        [parse_objct] consume key: name
        [parse_objct] parsed value: SString("A")
        [parse_objct] consume key: population
        [parse_objct] parsed value: Number(1000.0)
        [parse_array] add Object({"name": SString("A"), "population": Number(1000.0)}) to array
        [parse_array] next token: Some(Comma)
        [parse_array] start parse_value: {
        [parse_objct] consume key: name
        [parse_objct] parsed value: SString("B")
        [parse_objct] consume key: population
        [parse_objct] parsed value: Number(30000.0)
        [parse_array] add Object({"population": Number(30000.0), "name": SString("B")}) to array
        [parse_array] next token: Some(ArrayEnd)
        consume array end...
        [parse_objct] parsed value: Array([Object({"name": SString("A"), "population": Number(1000.0)}), Object({"population": Number(30000.0), "name": SString("B")})])

        */
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
                    ("name".to_string(), JsonValue::SString("A".to_string())),
                    ("population".to_string(), JsonValue::Number(1000 as f64)),
                ])),
                JsonValue::Object(HashMap::from_iter([
                    ("name".to_string(), JsonValue::SString("B".to_string())),
                    ("population".to_string(), JsonValue::Number(30000 as f64)),
                ])),
            ]),
        );

        let expect_json_value = JsonValue::Object(expect_object);
        assert_eq!(expect_json_value, json_value);
        Ok(())
    }
}
