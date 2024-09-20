use json_parser::JsonValue;

fn main() {
    let json_value = JsonValue::Array(vec![
        JsonValue::Boolean(true),
        JsonValue::JString("Hello, World!".to_string()),
    ]);
    println!("{:?}", json_value);
}
