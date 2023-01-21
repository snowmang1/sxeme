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

#[allow(dead_code)] /* temporary */
fn get_prog(buf: &mut BufReader<File>) -> Result<Vec<String>, std::io::Error>{
    let mut prog: Vec<String> = vec![];
    match buf.has_data_left() {
        Ok(false) => Ok(vec![]),
        Err(x)    => Err(x),
        Ok(true)  => {
            for line in buf.lines() { /* loop to itterate on each character of BufReader */
                prog.push(line.unwrap());
            }
            Ok(prog)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_prog_open_file_test() {
        let f = match File::create_new("foo.txt"){ // create_new opens in read-write mode
            Ok(ok)   => ok,
            Err(err) => panic!("{err}"),
        };
        let result: Vec<String> = match get_prog(&mut BufReader::new(f)) {
            Ok(ok)   => ok,
            Err(err) => panic!("{err}"),
        };
        let expected: Vec<String> = vec![];
        assert_eq!(result, expected);
        match std::fs::remove_file("foo.txt") {
            Ok(ok)   => ok,
            Err(err) => panic!("{err}")
        };
    }
}
