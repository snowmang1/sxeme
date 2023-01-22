/* 
 * scanner related interface
 * scanners will be implemented to read text in and format it into words/lines/symbols
 * & then give that data to the parser.
 *
 * SHOULD NOT BE THROWING ERRORS HERE.....EVER
 * 
 * NOTE
 * as this file gets bigger add files to this module
*/
use std::fs::File;
use std::io::{ BufRead, BufReader };

#[cfg(test)]
mod tests;

#[allow(dead_code)] /* temporary */
fn get_prog(filename: &String) -> Result<Vec<String>, std::io::Error>{
    let mut prog: Vec<String> = vec![];
    let buf = BufReader::new(match File::open(filename) {
        Ok(ok)   => ok,
        Err(err) => panic!("file failed to open: {err}"),
    });
    for line in buf.lines() {
        match line {
            Ok(s) if s.len() <= 0 => break,
            Err(err) => panic!("buf.lines Err: {err}"),
            _        => prog.append(&mut filter_spaces(&line.unwrap()).to_owned())
        };
    }
    Ok(prog)
}

fn filter_spaces(str: &String) -> Vec<String> {
    let mut ret: Vec<String> = Vec::new();
    let mut word = String::new();
    for c in str.chars() {
        if c == ' ' { ret.push(word.clone()); word.clear(); }
        else { word.push(c); }
    }
    match word.len() { // gaurd for trailing spaces
        0 => ret,
        _ => { ret.push(word.clone()); ret }
    }
}
