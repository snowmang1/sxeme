pub mod error_matrix;
#[cfg(test)]
mod tests;
mod token_stack;

use crate::scanner;
use error_matrix::ParserErrors;
use token_stack::TokenStack;

// TODO
// move parsers to own file
#[allow(dead_code)]
fn parse_driver(prog_name: String) -> Result<(), ParserErrors> {
    let mut stack: TokenStack = Default::default();
    let mut prog: Vec<String> = match scanner::get_prog(&prog_name) {
        Ok(ok) => ok,
        Err(err) => panic!("{err}"), // panic is passing io::error error up
    };

    // send to parsers
    arithmatic_parser(&mut prog, &mut stack)?;

    Ok(())
}

fn arithmatic_parser(prog: &mut Vec<String>, stack: &mut TokenStack) -> Result<(), ParserErrors> {
    let mut ers: Vec<ParserErrors> = vec![];
    while !prog.is_empty() {
        match stack.push(prog.remove(0_usize)) {
            Ok(_) => continue,
            Err(err) => ers.push(err),
        }
    }
    stack.block_finished()?;
    if !ers.is_empty() {
        Err(ers[0].clone())
    } else {
        Ok(())
    }
}
