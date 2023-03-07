use super::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub fn mk_svec(v: Vec<&str>) -> Vec<String> {
    let mut new: Vec<String> = Default::default();
    for s in v {
        new.push(String::from(s));
    }
    new
}

pub fn setup_arith_parsers(
    mut st: Vec<String>,
    mut tok: &mut TokenStack,
) -> Result<(), ParserErrors> {
    let result = arithmatic_parser(&mut st, &mut tok);

    let expected_stack: Vec<String> = vec![];
    assert_eq!(st, expected_stack);

    result
}

// --------------------------------------------- scanners functions
// creates file to feed scanner from given string
// this is string is then passed parse_driver -> scanner
pub fn feed_scanner_file(filename: &'static str, str: Option<String>) -> bool {
    match Path::exists(Path::new(filename)) {
        // guard incase test fails and is rerun
        true => cleanup(filename),
        false => (),
    };
    match str {
        None => {
            match File::create_new(filename) {
                // create_new opens in read-write mode
                Ok(ok) => ok,
                Err(err) => panic!("{err}"),
            };
            true
        }
        Some(str) => {
            let mut f = match File::create_new(filename) {
                // create_new opens in read-write mode
                Ok(ok) => ok,
                Err(err) => panic!("{err}"),
            };
            // write to file
            match f.write_all(str.as_bytes()) {
                Ok(ok) => ok,
                Err(err) => panic!("File writing Error: {err}"),
            };
            true
        }
    }
}

pub fn cleanup(filename: &'static str) {
    match std::fs::remove_file(filename) {
        Ok(ok) => ok,
        Err(err) => panic!("{err}"),
    };
}
