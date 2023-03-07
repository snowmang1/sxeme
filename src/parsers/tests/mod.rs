
mod helpers;

use helpers::*;
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
    let result = setup_arith_parsers(mk_svec(vec!["(", "(", "+", "1", "2", ")"]), &mut tok);
    assert_eq!(result, Err(ParserErrors::NoClosingParen));
}

#[test]
fn test_arith_no_opening_paren_error() {
    let mut tok: TokenStack = Default::default();
    let result = setup_arith_parsers(mk_svec(vec!["+", "1", "2", ")"]), &mut tok);
    assert_eq!(result, Err(ParserErrors::NoOpeningParen))
}

#[test]
fn test_arith_badsymbol() {
    let mut tok: TokenStack = Default::default();
    let result = setup_arith_parsers(mk_svec(vec!["(", "p", "1", "2", ")"]), &mut tok);
    assert_eq!(result, Err(ParserErrors::BadSymbol))
}

#[test]
fn test_arith_no_closing_paren() {
    let mut tok: TokenStack = Default::default();
    let result = setup_arith_parsers(mk_svec(vec!["(", "+", "1", "2"]), &mut tok);
    assert_eq!(result, Err(ParserErrors::NoClosingParen))
}

#[test]
fn test_parse_driver_arith() {
    feed_scanner_file("parse_driver_test.txt", None);
    let ret = parse_driver(String::from("parse_driver_test.txt"));
    assert_eq!(ret, Ok(()));
    cleanup("parse_driver_test.txt");
}

#[test]
#[should_panic]
fn test_parse_driver_failure() {
    parse_driver(String::from("file_does_exist")).unwrap();
}
