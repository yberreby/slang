/*
 * # Shitty Codegen
 *
 * Goal: implement extremely simple native code generation.
 *
 * ## A KISS approach to register allocation
 * Put everything on the stack, pop it before performing an operation, and push it again.
 * The register names can be hardcoded in the codegen function for each elementary operation.
 *
 * ## Labels
 * Generate labels programmatically: current context name + underscore + label number?
 *   => or UUID.
 *
 * ## Types
 * Only use integers at first.
 *
 *
 * ## Functions
 * Implement System V ABI ASAP in order to take advantage of FFI => hello libc!
 *
 */

use std::fmt;
use std::fmt::Write;

pub enum Value {
    Int(i64),
    // Register.
    Reg(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Int(i) => write!(f, "{}", i),
            Value::Reg(s) => write!(f, "{}", s),
        }
    }
}

type Word = usize;

pub struct CodeGenerator {
    asm_buffer: String,
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            asm_buffer: String::new(),
        }
    }

    pub fn into_string(self) -> String {
        self.asm_buffer
    }

    pub fn call(&mut self, symbol: &str, args: &[Value]) {
        // rdi, rsi, rdx, rcx, r8, r9, then stack

        let registers = ["rdi", "rsi", "rdx", "rcx", "r8", "r9"];

        let reg_args: &[Value];
        let stack_args: &[Value];

        if args.len() <= registers.len() {
            reg_args = args;
            stack_args = &[];
        } else {
            let (a, b) = args.split_at(registers.len());
            reg_args = a;
            stack_args = b;
        }

        for (arg, reg) in reg_args.iter().zip(registers.iter()) {
            self.mov_to_reg(&Value::Reg(reg.to_string()), arg);
        }

        for arg in stack_args {
            self.stack_push(arg);
        }

        self.write_instr("call", &[]);
    }

    /// Move the contents of the `src` register into `dst`.
    fn mov_to_reg(&mut self, dst: &Value, src: &Value) {
        self.write_instr("mov", &[dst, src]);
    }

    // iname: Instruction Name
    fn write_instr(&mut self, iname: &str, operands: &[&Value]) {
        self.asm_buffer.push_str(iname);

        match operands.split_last() {
            Some((last_op, other_ops)) => {
                for op in other_ops {
                    write!(&mut self.asm_buffer, " {},", op).unwrap();
                }

                write!(&mut self.asm_buffer, " {}", last_op).unwrap();
            }
            None => {}
        }

        self.asm_buffer.push_str("\n");
    }

    fn stack_push(&mut self, arg: &Value) {
        self.write_instr("push", &[arg])
    }

    fn label(&mut self, label: &str) {
        write!(&mut self.asm_buffer, "{}:\n", label).unwrap();
    }
}
