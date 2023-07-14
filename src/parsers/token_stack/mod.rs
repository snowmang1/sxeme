mod token;
mod translate_error;

use crate::parsers::error_matrix::ParserErrors;
use token::Token;
use token::Type;

use translate_error::TranslationErrors;

#[derive(Default, Debug, PartialEq, Clone)]
pub struct TokenStack {
    stack: Vec<Token>,
    delim_count: u32,
}

/* This stack works as a stack from the end only
 * The top of the stack = stack.len()-1
 * The stack should keep a custom type that denotes these
 * const: integer(i64), number(double32), complex(i32) < 0, real(u32), rational(u32)
 * handle: (), {}, []
 * key: +, -, \, *, modulo, ceiling, abs, denominator, square, quotient, gcd, expt, lcm
 */

#[allow(dead_code)]
impl TokenStack {
    // fn used to init tests
    #[cfg_attr(coverage_nightly, no_coverage)]
    fn init(st: Vec<String>, delim_count: u32) -> Result<Self, ParserErrors> {
        let mut stack: Vec<Token> = vec![];
        for lexeme in st {
            stack.push(Token::new(lexeme)?);
        }
        Ok(TokenStack { stack, delim_count })
    }

    // fn called for pushing each char
    pub fn push(&mut self, lex: String) -> Result<(), ParserErrors> {
        match lex.as_str() {
            "(" => {
                // state 1
                self.stack.push(Token::new(lex)?);
                self.delim_count += 1;
                Ok(())
            }
            ")" => {
                // state 5 reduce
                let mut accum = String::from(")");
                let mut err: ParserErrors = ParserErrors::Good;
                loop {
                    if self.stack.is_empty() {
                        err = ParserErrors::NoOpeningParen;
                        break;
                    }
                    match self.stack[self.stack.len() - 1].as_str() {
                        "(" => {
                            accum.insert(0, '(');
                            self.stack.pop();
                            self.stack.push(Token::formed(accum, Type::Number));
                            break;
                        }
                        ind => {
                            accum.insert_str(0, ind);
                            self.stack.pop();
                        }
                    }
                }
                if self.delim_count >= 1 {
                    self.delim_count -= 1;
                }
                match err {
                    ParserErrors::Good => Ok(()),
                    e => Err(e),
                }
            }
            _ => {
                self.stack.push(Token::new(lex)?);
                Ok(())
            }
        }
    }

    /*
     * This function returns a string that will be the eventual ouput llvm ir file
    */
    fn translate(&mut self) -> Result<String, TranslationErrors> {
        let top : Token = match self.pop_tok() {
            Ok(s) => s,
            Err(_) => panic!("token translation error")
        };
        top.translate(&mut (String::from('x'), 0))
    }

    // function called after block is finished or eof
    pub fn block_finished(&self) -> Result<(), ParserErrors> {
        if self.delim_count > 0 {
            Err(ParserErrors::NoClosingParen)
        } else {
            Ok(())
        }
    }

    fn pop_tok(&mut self) -> Result<Token, ParserErrors> {
        match self.stack.pop() {
            None => Err(ParserErrors::TokenStackEmpty),
            Some(s) => Ok(s),
        }
    }
    pub fn pop(&mut self) -> Result<String, ParserErrors> {
        match self.stack.pop() {
            None => Err(ParserErrors::TokenStackEmpty),
            Some(s) => Ok(String::from(s.as_str())),
        }
    }
    pub fn len(&self) -> usize {
        self.stack.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn reduction_to_number_2() {
        let mut stack = TokenStack::init(vec![], 0).unwrap();
        stack.push(String::from("(")).unwrap();
        stack.push(String::from("+")).unwrap();
        stack.push(String::from("1")).unwrap();
        stack.push(String::from("2")).unwrap();
        stack.push(String::from(")")).unwrap();
        let ret_tok: Token = stack.pop_tok().unwrap();
        assert_eq!(
            ret_tok,
            Token::formed(String::from("(+12)"), Type::Number)
        )
    }
    #[test]
    fn reduction_to_number_5() {
        let mut stack = TokenStack::init(vec![], 0).unwrap();
        stack.push(String::from("(")).unwrap();
        stack.push(String::from("+")).unwrap();
        stack.push(String::from("1")).unwrap();
        stack.push(String::from("2")).unwrap();
        stack.push(String::from("3")).unwrap();
        stack.push(String::from("4")).unwrap();
        stack.push(String::from("5")).unwrap();
        stack.push(String::from(")")).unwrap();
        let ret_tok: Token = stack.pop_tok().unwrap();
        assert_eq!(
            ret_tok,
            Token::formed(String::from("(+12345)"), Type::Number)
        )
    }

    #[test]
    fn pop_empty_stack() {
        let mut stack = TokenStack::init(vec![], 0).unwrap();
        assert_eq!(stack.pop(), Err(ParserErrors::TokenStackEmpty));
        assert_eq!(stack.pop_tok(), Err(ParserErrors::TokenStackEmpty))
    }

    #[test]
    fn translation_addition_0() {
        let mut stack = TokenStack::init(vec![], 0).expect("stack failed to initialize");
        stack.push(String::from("(")).unwrap();
        stack.push(String::from("+")).unwrap();
        stack.push(String::from("1")).unwrap();
        stack.push(String::from("2")).unwrap();
        stack.push(String::from(")")).unwrap();
        let assembly: String = stack.pop_tok().expect("token pop fail")
            .translate(&mut (String::from("x"), 0)).expect("token translate fail");
        println!("{}", assembly);
    }
}
