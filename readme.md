# json-rs

## Run in Command line

```sh
json-parser (main) [1]> cargo run .
   Compiling json-parser v0.1.0
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.79s
     Running `target/debug/json-parser .`
>>{"date":"2024-09-22"}
tokens:
[ObjectStart, String("date"), Colon, String("2024-09-22"), ObjectEnd]
parsed json:
Object({"date": JString("2024-09-22")})

>>{"idea_list":["Caculator", "Simple Interpreter", "Image Edtior", "Twitter Clone - Axum Web Development"]}
tokens:
[ObjectStart, String("idea_list"), Colon, ArrayStart, String("Caculator"), Comma, String("Simple Interpreter"), Comma, String("Image Edtior"), Comma, String("Twitter Clone - Axum Web Development"), ArrayEnd, ObjectEnd]
parsed json:
Object({"idea_list": Array([JString("Caculator"), JString("Simple Interpreter"), JString("Image Edtior"), JString("Twitter Clone - Axum Web Development")])})

>>["Caculator", "Simple Interpreter", "Image Edtior", "Twitter Clone - Axum Web Development"]
tokens:
[ArrayStart, String("Caculator"), Comma, String("Simple Interpreter"), Comma, String("Image Edtior"), Comma, String("Twitter Clone - Axum Web Development"), ArrayEnd]
parsed json:
Array([JString("Caculator"), JString("Simple Interpreter"), JString("Image Edtior"), JString("Twitter Clone - Axum Web Development")])

>>{"nice";"good"}
unsupported char: ;
```
