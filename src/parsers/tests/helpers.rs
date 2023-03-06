use super::*;

pub fn mk_svec(v: Vec<&str>) -> Vec<String> {
    let mut new: Vec<String> = Default::default();
    for s in v {
        new.push(String::from(s));
    }
    new
}

pub fn setup_arith_parsers(mut st: Vec<String>, mut tok: &mut TokenStack) -> Result<(), ParserErrors> {
    let result = arithmatic_parser(&mut st, &mut tok);

    let expected_stack: Vec<String> = vec![];
    assert_eq!(st, expected_stack);

    result
}

