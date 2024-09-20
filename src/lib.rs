use core::fmt;
use std::collections::HashMap;

pub mod errors;
pub mod parser;
pub mod scanner;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    ObjectStart,    // {
    ObjectEnd,      // }
    ArrayStart,     // [
    ArrayEnd,       //]
    String(String), // "key" or "value"
    Number(f64),    // the num
    Bool(bool),     // true or false
    Comma,          // ,
    Colon,          // :
    WhiteSpace,     // ws \r \t \n
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    ObjectStart, // {
    ObjectEnd,   // }
    ArrayStart,  // [
    ArrayEnd,    //]
    String,      // "key" or "value"
    Number,      // the num
    Bool,        // true or false
    Comma,       // ,
    Colon,       // :
}

#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    Object(HashMap<String, JsonValue>),
    JString(String),
    Array(Vec<JsonValue>),
    Boolean(bool),
    Number(f64),
    Null,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ObjectStart => write!(f, "{}", "{"),
            Self::ObjectEnd => write!(f, "{}", "}"),
            Self::ArrayStart => write!(f, "["),
            Self::ArrayEnd => write!(f, "]"),
            Self::String(v) => write!(f, "{}", v),
            Self::Number(v) => write!(f, "{}", v),
            Self::Bool(v) => write!(f, "{}", v),
            Self::Null => write!(f, "null"),
            Self::Comma => write!(f, ","),
            Self::Colon => write!(f, ":"),
            _ => write!(f, " "),
        }
    }
}
