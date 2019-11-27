pub mod ast;

#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub grammar); // synthesized by LALRPOP

#[test]
fn trivial_fn_decl() {
    assert!(grammar::FuncParser::new().parse("fn hello() { (); }").is_ok());
}
