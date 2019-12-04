pub mod ast;

#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub grammar); // synthesized by LALRPOP

#[test]
fn trivial_fn_decl() {
    grammar::FuncParser::new().parse("fn hello() {}").unwrap();
    grammar::FuncParser::new().parse("fn hello(a: i64, b: f32) { (); (); }").unwrap();
}


