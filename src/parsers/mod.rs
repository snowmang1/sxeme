#[cfg(test)]
mod tests;
mod token_stack;
pub mod error_matrix;

use crate::scanner;
use token_stack::TokenStack;
use error_matrix::ParserErrors;

// TODO
// move types to own dir
// move parsers to own file
#[allow(dead_code)]
fn parse_driver(prog_name: String) -> Result<(), ParserErrors>{
    let mut stack: TokenStack = Default::default();
    let mut prog: Vec<String> = match scanner::get_prog(&prog_name) {
        Ok(ok)   => ok,
        Err(err) => panic!("{err}") // panic is passing io::error error up
    };

    // send to parsers
    match arithmatic_parser(&mut prog, &mut stack) {
        Err(_) => panic!("arithmatic Error: NEEDS PARSERERROR INTEGRATION"),
        Ok(_)   => Ok(())
    }
}

fn arithmatic_parser(prog: &mut Vec<String>, stack: &mut TokenStack) -> Result<(), ParserErrors> {
    let _ers: Vec<ParserErrors> = vec![];
    while !prog.is_empty() {
        stack.push(prog.remove(0 as usize))?
    }
    Ok(())
}
