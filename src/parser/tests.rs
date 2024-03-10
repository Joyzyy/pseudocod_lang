use std::{
    borrow::Borrow,
    cell::{Ref, RefCell},
    ops::Deref,
    rc::Rc,
};

use crate::{
    ast::{LetStatement, Program, ReturnStatement, Statement},
    lexer::Lexer,
};

use super::Parser;

#[test]
fn test_one() {
    let input = r#"
    let x 5;
    let = 10;
    let 838383;
  "#
    .to_string();

    let lexer = Lexer::new(input);
    let mut p = Parser::new(lexer);
    let program = p.parse_program().unwrap();
    let errors = p.errors();
    dbg!(errors);

    assert_eq!(program.statements.len(), 3);

    // println!("{:?}", program.statements.len());

    let tests = vec!["x".to_string(), "y".to_string(), "foobar".to_string()];
    for (i, tt) in tests.iter().enumerate() {
        let stmt = program.statements.get(i).unwrap();
        let stmt = stmt.as_ref().borrow();
        let let_stmt = stmt.as_any().downcast_ref::<LetStatement>().unwrap();
        assert_eq!(&let_stmt.name.as_ref().unwrap().value, tt);
    }
}

#[test]
fn return_statements() {
    let input = r#"
    return 5;
    return 10;
    return 9993322;
  "#
    .to_string();

    let lexer = Lexer::new(input);
    let mut p = Parser::new(lexer);
    let program = p.parse_program().unwrap();
    let errors = p.errors();
    dbg!(errors);

    assert_eq!(program.statements.len(), 3);

    let tests = vec![5, 10, 9993322];
    for (i, _) in tests.iter().enumerate() {
        let stmt = program.statements.get(i).unwrap();
        let stmt = stmt.as_ref().borrow();
        let return_stmt = stmt.as_any().downcast_ref::<ReturnStatement>().unwrap();

        assert_eq!(return_stmt.token.literal, "return");
    }
}
