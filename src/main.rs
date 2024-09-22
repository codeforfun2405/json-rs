use std::io::{self, BufRead, Write};

use json_parser::{parser::Parser, scanner::Scanner};

fn main() -> anyhow::Result<()> {
    loop {
        match rep() {
            Err(e) => println!("{}", e),
            _ => {}
        }
    }
}

fn rep() -> anyhow::Result<()> {
    let mut stdin_lock = io::stdin().lock();
    print!(">>");
    io::stdout().flush()?;

    let mut buf = String::new();
    stdin_lock.read_line(&mut buf)?;

    let mut scanner = Scanner::new(buf);
    let tokens = scanner.scan()?;
    println!("tokens:");
    println!("{:?}", tokens);

    let mut json_parser = Parser::new(tokens);
    let json_value = json_parser.parse()?;

    println!("parsed json:");
    println!("{:?}", json_value);
    Ok(())
}
