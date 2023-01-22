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
