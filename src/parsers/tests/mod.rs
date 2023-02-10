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
fn test_arith_reduce_1() {
    let mut test_prog: Vec<String> = mk_svec(vec!["(", "+", "1", "2", ")"]);
    let mut tok: TokenStack = Default::default();
    let expected_prog: Vec<String> = vec![];
    let result = arithmatic_parser(&mut test_prog, &mut tok);
    assert_eq!(result, Ok(()));
    assert_eq!(test_prog, expected_prog);
    let (top,_) = tok.stack[0].to_owned();
    assert_eq!(top, String::from("(+12)"))
}
