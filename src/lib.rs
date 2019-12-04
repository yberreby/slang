pub mod ast;

#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub grammar); // synthesized by LALRPOP

#[test]
fn trivial_fn_decl() {
    grammar::FuncParser::new().parse("fn hello() {}").unwrap();
}


#[test]
fn basic_fn_decl() {
    grammar::FuncParser::new().parse("fn hello(a: i64, b: f32) { (); (); }").unwrap();
}


#[test]
fn statement_list_empty_stmt() {
    grammar::StatementListParser::new().parse("(); ();").unwrap();
}


#[test]
fn statement_list_simple_assignment() {
    grammar::StatementListParser::new().parse("a = 5;").unwrap();
}
