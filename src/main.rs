extern crate slang;
use slang::{ast,codegen,grammar};

use std::io;
use std::io::Read;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let toplevel = grammar::ProgramParser::new().parse(&buffer).unwrap();

    println!("{}", codegen::generate(toplevel));

    Ok(())
}
