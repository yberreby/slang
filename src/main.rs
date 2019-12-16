

extern crate slang;
use slang::{ast,codegen,grammar};

use std::io;
use std::io::Read;

mod shitty_codegen;
use shitty_codegen::{CodeGenerator, Value};

fn main() -> io::Result<()> {
    
    let mut cg = CodeGenerator::new();
    cg.call("glibc_printf", &[Value::Int(12), Value::Reg("eax".to_string()),
        Value::Int(1),
        Value::Int(2),
        Value::Int(1),
        Value::Int(1),
        Value::Int(1),
        Value::Int(2),
        Value::Int(3),
        Value::Int(4),
        Value::Int(1),
    ]);
    println!("{}", cg.into_string());

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let toplevel = grammar::ProgramParser::new().parse(&buffer).unwrap();

    println!("{}", codegen::generate(toplevel));

    Ok(())
}
