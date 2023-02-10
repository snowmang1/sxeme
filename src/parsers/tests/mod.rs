use super::*;

fn mk_svec(v: Vec<&str>) -> Vec<String> {
    let mut new: Vec<String> = Default::default();
    for s in v {
        new.push(String::from(s));
    }
    new
}

#[test]
fn test_arith_empty() {
    let mut test_prog: Vec<String> = mk_svec(vec![]);
    let mut tok: TokenStack = Default::default();
    let expected_prog: Vec<String> = vec![];
    let result = arithmatic_parser(&mut test_prog, &mut tok);
    assert_eq!(result, Ok(()));
    assert_eq!(test_prog, expected_prog);
}

#[test]
fn test_arith_drain() {
    let mut test_prog: Vec<String> = mk_svec(vec!["(", "+", "1", "1", ")"]);
    let mut tok: TokenStack = Default::default();
    let expected_prog: Vec<String> = vec![];
    let result = arithmatic_parser(&mut test_prog, &mut tok);
    assert_eq!(result, Ok(()));
    assert_eq!(test_prog, expected_prog);
    assert!(tok.stack.len() != 0);
}

#[test]
fn test_arith_reduce_1() {
    let mut test_prog: Vec<String> = mk_svec(vec!["(", "+", "1", "2", ")"]);
    let mut tok: TokenStack = Default::default();
    let expected_prog: Vec<String> = vec![];
    let result = arithmatic_parser(&mut test_prog, &mut tok);
    assert_eq!(result, Ok(()));
    assert_eq!(test_prog, expected_prog);
    let top = tok.stack[0].to_owned();
    assert_eq!(top, String::from("(+12)"))
}

#[test]
fn test_arith_ignore_extra_paren() {
    let mut test_prog: Vec<String> = mk_svec(vec!["(", "(", "+", "1", "2", ")"]);
    let mut tok: TokenStack = Default::default();
    arithmatic_parser(&mut test_prog, &mut tok).unwrap();
    assert_eq!("(", tok.stack[0].as_str());
    assert_eq!("(+12)", tok.stack[1].as_str());
}

#[test]
fn test_arith_no_closing_paren_error() {
    let mut test_prog: Vec<String> = mk_svec(vec!["(", "(", "+", "1", "2"]);
    let mut tok: TokenStack = Default::default();
    let result = arithmatic_parser(&mut test_prog, &mut tok).unwrap();

}
