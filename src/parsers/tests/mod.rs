
mod helpers;

use helpers::{ setup_arith_parsers, mk_svec };
use super::*;

#[test]
fn test_arith_empty() {
    let result = setup_arith_parsers(mk_svec(vec![]), &mut Default::default());
    assert_eq!(result, Ok(()));
}

#[test]
fn test_arith_drain() {
    let mut tok: TokenStack = Default::default();
    let result = setup_arith_parsers(mk_svec(vec!["(", "+", "1", "1", ")"]), &mut tok);
    assert_eq!(result, Ok(()));
    assert!(tok.len() != 0);
}

#[test]
fn test_arith_reduce_1() {
    let mut tok: TokenStack = Default::default();
    let result = setup_arith_parsers(mk_svec(vec!["(", "+", "1", "2", ")"]), &mut tok);
    assert_eq!(result, Ok(()));
    let top = tok.pop().unwrap();           // get the top of the stack
    assert_eq!(top, String::from("(+12)"))
}

#[test]
fn test_arith_ignore_extra_paren() {
    let mut tok: TokenStack = Default::default();
    setup_arith_parsers(mk_svec(vec!["(", "(", "+", "1", "2", ")"]), &mut tok).unwrap();
    assert_eq!(tok.pop().unwrap().as_str(), "(+12)");
    assert_eq!(tok.pop().unwrap().as_str(), "(");
}

#[test]
fn test_arith_no_closing_paren_error() {
    let mut tok: TokenStack = Default::default();
    let result = setup_arith_parsers(mk_svec(vec!["+", "1", "2", ")"]), &mut tok);
    assert_eq!(result, Err(ParserErrors::NoOpeningParen))
}
