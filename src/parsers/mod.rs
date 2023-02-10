#[cfg(test)]
mod tests;

use crate::scanner;

// TODO
// move types to own dir
// move parsers to own file

#[derive(Default, Debug, PartialEq, Clone)]
struct TokenStack { stack: Vec<String>, open: u32 }

#[allow(dead_code)]
impl TokenStack {
    fn init(st: Vec<String>, o: u32) -> Self { TokenStack { stack: st, open: o } }
    // above is DEBUG use only
    fn push(&mut self, lex: String) -> Result<(), String>{
        match lex.as_str() {
            "(" => { // state 1
                self.stack.push(lex);
                self.open+=1;
                Ok(())
            }
            ")" => { // state 5 reduce
                let mut accum = String::from(")");
                loop {
                    match self.stack.pop() {
                        Some(s) => { match s.as_str() {
                            "(" => { accum.insert(0, '('); self.stack.push(accum); break },
                            _   => () // Errors
                        }; accum.insert_str(0, s.as_str()); }
                        None    => () // Errors
                    }
                }
                self.open-=1;
                Ok(())
            }
            "+" => { // state 2 operation
                self.stack.push(lex);
                Ok(())
            }
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => { // state 3
                self.stack.push(lex);
                Ok(())
            }
            x   => Err(format!("unknown symbol {x}"))
        }
    }
}

#[allow(dead_code)]
fn parse_driver(prog_name: String) -> Result<(), &'static str>{
    let mut stack: TokenStack = Default::default();
    let mut prog: Vec<String> = match scanner::get_prog(&prog_name) {
        Ok(ok)   => ok,
        Err(err) => panic!("{err}") // panic is passing io::error error up
    };

    // send to parsers
    match arithmatic_parser(&mut prog, &mut stack) {
        Err(err) => panic!("arithmatic Error: {err}"),
        Ok(_)   => Ok(())
    }
}

#[allow(dead_code)]
fn arithmatic_parser(prog: &mut Vec<String>, stack: &mut TokenStack) -> Result<(), String> {
    let mut ers: Vec<String> = Default::default();
    while !prog.is_empty() {
        match stack.push(prog.remove(0 as usize)) {
            Ok(_)    => (),
            Err(err) => ers.push(err)
        };
    }
    Ok(())
}
