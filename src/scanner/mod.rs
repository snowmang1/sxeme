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
    let mut comment: bool = false;
    let buf = BufReader::new(match File::open(filename) {
        Ok(ok)   => ok,
        Err(err) => panic!("file failed to open: {err}"),
    });
    for line in buf.lines() {
        match comment {
            true => {
                
            },
            false => {
                // check comment
                match line {
                    Ok(s) if s.len() <= 0 => break,
                    Err(err) => panic!("buf.lines Err: {err}"),
                    _        => {
                        let (v, temp_comment) = &mut filter_spaces(&line.unwrap()).to_owned();
                        comment = temp_comment.to_owned();
                        prog.append(v)
                    }
                };
            }
        }
    }
    Ok(prog)
}

fn filter_spaces(str: &String) -> (Vec<String>, bool) {
    let mut ret: Vec<String> = Vec::new();  // vector of lexeme
    let mut word = String::new();           // lexeme buffer
    let mut comment_flag: u8 = 0;           // flag for block comments
    // 0 -> no comment | 1 -> beging of block comment | 2 -> mid comment | 3 -> beging of closing block comment
    for c in str.chars() {
        match c {
            ' ' => { if comment_flag != 2 { ret.push(word.clone()); word.clear(); comment_flag = 0; } },
            ';' => { if comment_flag != 2 { break; } },
            '#' => {
                if comment_flag < 2 { comment_flag = 1; word.push(c); }
                else if comment_flag == 3 { comment_flag = 0; }
            }
            '|' => {
                if comment_flag == 1 { comment_flag = 2; word.pop(); }
                else if comment_flag == 2 { comment_flag = 3; }
            }
            _   => { if comment_flag != 2 { word.push(c); comment_flag = 0; } }
        }
    }
    match word.len() { // gaurd for trailing spaces
        0 => (ret, match comment_flag { 2 => true, _ => false }),
        _ => { ret.push(word.clone()); (ret, match comment_flag { 2 => true, _ => false }) }
    }
}
