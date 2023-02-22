use super::error_matrix::ParserErrors;

#[derive(Default, Debug, PartialEq, Clone)]
pub struct TokenStack { stack: Vec<String>, open: u32 }
/* This stack works as a stack from the end only
 * The top of the stack = stack.len()-1
 */

#[allow(dead_code)]
impl TokenStack {
    fn init(st: Vec<String>, o: u32) -> Self { TokenStack { stack: st, open: o } }
    // above is DEBUG use only
    pub fn push(&mut self, lex: String) -> Result<(), ParserErrors>{
        match lex.as_str() {
            "(" => { // state 1
                self.stack.push(lex);
                self.open+=1;
                Ok(())
            }
            ")" => { // state 5 reduce
                let mut accum = String::from(")");
                let mut err: ParserErrors = ParserErrors::Good;
                loop {
                    if self.stack.is_empty() { err = ParserErrors::NoOpeningParen; break }
                    match self.stack[(self.stack.len()-1) as usize].as_str() {
                        "(" => { accum.insert(0, '('); self.stack.pop(); self.stack.push(accum); break },
                        ind => { accum.insert_str(0, ind); self.stack.pop(); }
                    }
                }
                if self.open >= 1 { self.open-=1; }
                match err {
                    ParserErrors::Good => Ok(()),
                    e => Err(e)
                }
            }
            "+" => { // state 2 operation
                self.stack.push(lex);
                Ok(())
            }
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => { // state 3
                self.stack.push(lex);
                Ok(())
            }
            _   => Err(ParserErrors::UnknownSymbol(lex))
        }
    }

    pub fn pop(&mut self) -> String {
        match self.stack.pop() { None => String::from("stack empty"), Some(s) => s }
    }
    pub fn len(&self) -> usize {
        self.stack.len()
    }
}

