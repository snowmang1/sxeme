mod helpers;

use helpers::*;
use super::*;

#[test]
fn get_file_return_vec() {
    let result = feed_scanner_file("scanner_test_file.txt", None);
    let expected: Vec<String> = vec![];
    assert_eq!(result, expected);
    cleanup("scanner_test_file.txt");
}

#[test]
fn scan_1_line() {
    let result = feed_scanner_file("scanner_test_file1.txt", Some(String::from("hello world!")));
    let expected: Vec<String> = vec![String::from("hello"), String::from("world!")];
    assert_eq!(result, expected);
    cleanup("scanner_test_file1.txt");
}

#[test]
fn scan_2_line() {
    let result = feed_scanner_file("scanner_test_file2.txt", Some(String::from("hello world!\nThis is Sxeme a compiler for Scheme written in Rust ")));
    let expected: Vec<String> = vec![String::from("hello"), String::from("world!"), String::from("This"), String::from("is"),
        String::from("Sxeme"), String::from("a"), String::from("compiler"), String::from("for"), String::from("Scheme"),
        String::from("written"), String::from("in"), String::from("Rust")];
    assert_eq!(result, expected);
    cleanup("scanner_test_file2.txt");
}

#[test]
fn scan_multi_lines() {
    let result = feed_scanner_file("scanner_test_file3.txt", Some(String::from("hello world!\nThis is Sxeme a compiler \n for Scheme written in Rust ")));
    let expected: Vec<String> = vec![String::from("hello"), String::from("world!"), String::from("This"), String::from("is"),
        String::from("Sxeme"), String::from("a"), String::from("compiler"), String::from(""), String::from("for"), String::from("Scheme"),
        String::from("written"), String::from("in"), String::from("Rust")];
    assert_eq!(result, expected);
    cleanup("scanner_test_file3.txt");
}

#[test]
fn ignore_line_comments() {
    let result = feed_scanner_file("scanner_test_file4.txt", Some(String::from("; hello world!")));
    let expected: Vec<String> = vec![];
    assert_eq!(result, expected);
    cleanup("scanner_test_file4.txt");
}

#[test]
fn ignore_line_comments_with_output() {
    let result = feed_scanner_file("scanner_test_file5.txt", Some(String::from("Sxeme is awesome ; hello world!")));
    let expected: Vec<String> = vec![String::from("Sxeme"), String::from("is"), String::from("awesome")];
    assert_eq!(result, expected);
    cleanup("scanner_test_file5.txt");
}

#[test]
fn ignore_block_comment() {
    let result = feed_scanner_file("scanner_test_file6.txt", Some(String::from("#| hello world! \n this is a comment |#")));
    let expected: Vec<String> = vec![];
    assert_eq!(result, expected);
    cleanup("scanner_test_file6.txt");
}

#[test]
fn ignore_block_comment_with_output() {
    let result = feed_scanner_file("scanner_test_file7.txt", Some(String::from("Sxeme is awesome #| hello world! \n this is a comment |#")));
    let expected: Vec<String> = vec![String::from("Sxeme"), String::from("is"), String::from("awesome")];
    assert_eq!(result, expected);
    cleanup("scanner_test_file7.txt");
}

#[test]
fn block_comment_repeat_octet() {
    let result = feed_scanner_file("scanner_test_file8.txt", Some(String::from("#^ #! ########| nope |# #%")));
    let expected: Vec<String> = vec![String::from("#^"), String::from("#!"), String::from("#######"), String::from("#%")];
    assert_eq!(result, expected);
    cleanup("scanner_test_file8.txt");
}
