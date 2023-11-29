use clap::Parser;
use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
};

mod scanner;
mod token;
mod err;
mod parser;
mod interpreter;
mod environment;

#[derive(Parser)]
struct Cli {
    file_name: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    if let Some(file_name) = cli.file_name {
        let content = fs::read_to_string(file_name).expect("file not found");
        run(&content).unwrap_or_else(|e| eprintln!("{}", e));
    } else {
        loop {
            print!("> ");
            io::stdout().flush().unwrap();
            let mut buffer = String::new();
            io::stdin()
                .read_line(&mut buffer)
                .expect("failed to read line");
            run(&buffer).unwrap_or_else(|e| eprintln!("{}", e));
        }
    }
}

fn run(source: &str) -> Result<(), err::Error> {
    let a = scanner::scan_tokens(source).unwrap();
    dbg!(&a);
    let mut parser = parser::Parser::new(a);
    let d = parser.parse()?;
    dbg!(&d);
    let mut interpreter = interpreter::Interpreter::new();
    for s in d {
        interpreter.eval_stmt(s)?;
    }
    // let v = interpreter::eval(e)?;
    // dbg!(&v);
    Ok(())
}
