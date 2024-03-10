use std::ops::Deref;

use crate::{ast::Program, lexer::Lexer};

use super::Parser;

#[test]
fn test_one() {
    let input = r#"
    let x = 5;
    let y = 10;
    let foobar = 838383;
  "#
    .to_string();

    let mut lexer = Lexer::new(input);
    let mut p = Parser::new(Box::new(lexer));
    let mut program = p.parse_program();

    if program.is_none() {
        panic!("parse_program() is none");
    }
    if program.as_ref().unwrap().as_ref().statements.len() != 3 {
        panic!("program.statements does not contain 3 statements");
    }

    let tests = vec!["x", "y", "foobar'"];
    for (i, _testt) in tests.iter().enumerate() {
        let stmt = program.as_ref().clone().unwrap().statements[i].as_ref();
        println!("{:?}", stmt.statement_node());
    }
}
