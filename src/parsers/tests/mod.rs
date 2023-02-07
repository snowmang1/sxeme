use super::*;

#[test]
fn test_empty_prog() {
    let mut test_prog: Vec<String> = vec![];
    let result = arithmatic_parser(&mut test_prog, &mut vec![]);
    assert_eq!(result, Err("empty file"))
}

#[test]
fn test_addition_no_nesting() {
    let mut test_prog = vec![String::from("("), String::from("+"), String::from("1"), String::from("2"), String::from(")")];
    let result = arithmatic_parser(&mut test_prog, &mut vec![]);
    let expected_stack = vec![String::from("(+12)")];
    assert_eq!( result, Ok((vec![], expected_stack)) )
}
