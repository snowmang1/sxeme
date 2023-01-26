#[cfg(test)]
mod tests;

use crate::scanner;

#[allow(dead_code)]
fn parse_driver(prog_name: String) {
    let _prog: Vec<String> = match scanner::get_prog(&prog_name) {
        Ok(ok)   => ok,
        Err(err) => panic!("{err}")
    };

}
