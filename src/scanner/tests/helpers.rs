use super::*;
use std::fs::File;
use std::io::Write;
use std::path::Path;

// --------------------------------------------- helper functions
pub fn feed_scanner_file(filename: &'static str, str: Option<String>) -> Vec<String> {
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
            let result: Vec<String> = match get_prog(&String::from(filename)) {
                Ok(ok) => ok,
                Err(err) => panic!("{err}"),
            };
            result
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
            let result: Vec<String> = match get_prog(&String::from(filename)) {
                Ok(ok) => ok,
                Err(err) => panic!("{err}"),
            };
            result
        }
    }
}

pub fn cleanup(filename: &'static str) {
    match std::fs::remove_file(filename) {
        Ok(ok) => ok,
        Err(err) => panic!("{err}"),
    };
}
