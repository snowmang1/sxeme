#[cfg(test)]
mod tests;

use crate::scanner;

// TODO
// move types to own dir
// move parsers to own file

#[derive(Default, Debug, PartialEq, Clone)]
struct TokenStack { stack: Vec<(String, u32)>, position: u32, open: u32 }

#[allow(dead_code)]
impl TokenStack {
    fn init(pos: u32, st: Vec<(String, u32)>, o: u32) -> Self { TokenStack { position: pos, stack: st, open: o } }
    fn print(&self) { println!("start:"); for (s,_) in self.stack.clone() { println!("{s}"); } }
    // above is DEBUG use only
    fn push(&mut self, lex: String) -> Result<u32, String>{
        match lex.as_str() {
            "(" => { // state 1
                self.stack.push((lex, self.position));
                self.open+=1;
                Ok(self.position)
            }
            ")" => { // state 5 reduce
                self.stack.push((lex, self.position));
                self.position=0; self.open-=1;
                Ok(self.position)
            }
            "+" => { // state 2 operation
                self.stack.push((lex, self.position));
                self.position+=2;
                Ok(self.position)
            }
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => { // state 3
                self.stack.push((lex, self.position));
                Ok(self.position)
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
    let mut pos: u32 = 0;
    let mut ers: Vec<String> = Default::default();
    while !prog.is_empty() {
        match stack.push(prog.remove(pos as usize)) { // Beware
            Ok(ok)   => {pos=ok; println!("{ok}")}
            Err(err) => ers.push(err)
        }
    }
    Ok(())
}
