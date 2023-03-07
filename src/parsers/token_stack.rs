use super::error_matrix::ParserErrors;


#[derive(Debug, PartialEq, Clone)]
enum Type { Number, Plus, Minus, Divide, Multiply, Handle }

#[derive(Debug, PartialEq, Clone)]
struct Token { lexeme: String, my_type: Type }
/* TODO
 * - overwrite insert & insert_str to create seamless replace
 * - lines 40 & 41
 */
impl Token {
    fn new(lex: String) -> Result<Token, ParserErrors> {
        let mut err: Vec<ParserErrors> = vec![];
        let ret: Option<Token> = match lex.clone().pop().unwrap() {
            '0' | '1' | '2' | '3' | '4' | '5' |
                '6' | '7' | '8' | '9' => Some (Token { lexeme: lex, my_type: Type::Number}),
            '(' | ')' => Some (Token { lexeme: lex, my_type: Type::Handle}),
            '+' => Some (Token { lexeme: lex, my_type: Type::Plus }),
            '-' => Some (Token { lexeme: lex, my_type: Type::Minus }),
            '/' => Some (Token { lexeme: lex, my_type: Type::Divide }),
            '*' => Some (Token { lexeme: lex, my_type: Type::Multiply }),
            _ => { err.push(ParserErrors::BadSymbol); None } // CHANGE
        };
        match ret { Some(ok) => Ok(ok), _ => Err(err.pop().unwrap()) }
    }

    fn formed(lex: String, output_type: Type) -> Token {
        Token{ lexeme: lex, my_type: output_type }
    }
    fn as_str(&self) -> &str { self.lexeme.as_str() }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct TokenStack { stack: Vec<Token>, delim_count: u32 }
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
    pub fn push(&mut self, lex: String) -> Result<(), ParserErrors>{
        match lex.as_str() {
            "(" => { // state 1
                self.stack.push(Token::new(lex)?);
                self.delim_count+=1;
                Ok(())
            }
            ")" => { // state 5 reduce
                let mut accum = String::from(")");
                let mut err: ParserErrors = ParserErrors::Good;
                loop {
                    if self.stack.is_empty() { err = ParserErrors::NoOpeningParen; break }
                    match self.stack[self.stack.len()-1].as_str() {
                        "(" => { accum.insert(0, '('); self.stack.pop(); self.stack.push(Token::formed(accum, Type::Number)); break },
                        ind => { accum.insert_str(0, ind); self.stack.pop(); }
                    }
                }
                if self.delim_count >= 1 { self.delim_count-=1; }
                match err {
                    ParserErrors::Good => Ok(()),
                    e => Err(e)
                }
            }
            _   => {
                self.stack.push(Token::new(lex)?);
                Ok(())
            }
        }
    }

    // function called after block is finished or eof
    pub fn block_finished(&self) -> Result<(), ParserErrors> {
        if self.delim_count > 0 { Err(ParserErrors::NoClosingParen) }
        else { Ok(()) }
    }

    fn pop_tok(&mut self) -> Result<Token, ParserErrors> {
        match self.stack.pop() { None => Err(ParserErrors::TokenStackEmpty), Some(s) => Ok(s) }
    }
    pub fn pop(&mut self) -> Result<String, ParserErrors> {
        match self.stack.pop() { None => Err(ParserErrors::TokenStackEmpty), Some(s) => Ok(String::from(s.as_str())) }
    }
    pub fn len(&self) -> usize {
        self.stack.len()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn type_number() {
        let new_token = Token::new(String::from("0"));
        assert_eq!(new_token.unwrap(), Token {lexeme: String::from("0"), my_type: Type::Number});
        let new_token = Token::new(String::from("1"));
        assert_eq!(new_token.unwrap(), Token {lexeme: String::from("1"), my_type: Type::Number});
        let new_token = Token::new(String::from("2"));
        assert_eq!(new_token.unwrap(), Token {lexeme: String::from("2"), my_type: Type::Number});
        let new_token = Token::new(String::from("3"));
        assert_eq!(new_token.unwrap(), Token {lexeme: String::from("3"), my_type: Type::Number});
        let new_token = Token::new(String::from("4"));
        assert_eq!(new_token.unwrap(), Token {lexeme: String::from("4"), my_type: Type::Number});
        let new_token = Token::new(String::from("5"));
        assert_eq!(new_token.unwrap(), Token {lexeme: String::from("5"), my_type: Type::Number});
        let new_token = Token::new(String::from("6"));
        assert_eq!(new_token.unwrap(), Token {lexeme: String::from("6"), my_type: Type::Number});
        let new_token = Token::new(String::from("7"));
        assert_eq!(new_token.unwrap(), Token {lexeme: String::from("7"), my_type: Type::Number});
        let new_token = Token::new(String::from("8"));
        assert_eq!(new_token.unwrap(), Token {lexeme: String::from("8"), my_type: Type::Number});
        let new_token = Token::new(String::from("9"));
        assert_eq!(new_token.unwrap(), Token {lexeme: String::from("9"), my_type: Type::Number});
    }

    #[test]
    fn type_plus() {
        let new_token = Token::new(String::from("+"));
        assert_eq!(new_token.unwrap(), Token {lexeme: String::from("+"), my_type: Type::Plus});
    }

    #[test]
    fn type_minus() {
        let new_token = Token::new(String::from("-"));
        assert_eq!(new_token.unwrap(), Token {lexeme: String::from("-"), my_type: Type::Minus});
    }

    #[test]
    fn type_divide() {
        let new_token = Token::new(String::from("/"));
        assert_eq!(new_token.unwrap(), Token {lexeme: String::from("/"), my_type: Type::Divide});
    }

    #[test]
    fn type_multiply() {
        let new_token = Token::new(String::from("*"));
        assert_eq!(new_token.unwrap(), Token {lexeme: String::from("*"), my_type: Type::Multiply});
    }

    #[test]
    fn type_handle() {
        let new_token = Token::new(String::from("("));
        assert_eq!(new_token.unwrap(), Token {lexeme: String::from("("), my_type: Type::Handle});
        let new_token = Token::new(String::from(")"));
        assert_eq!(new_token.unwrap(), Token {lexeme: String::from(")"), my_type: Type::Handle});
    }

    #[test]
    fn reduction_to_number_2() {
        let mut stack = TokenStack::init(vec![], 0).unwrap();
        stack.push(String::from("(")).unwrap();
        stack.push(String::from("+")).unwrap();
        stack.push(String::from("1")).unwrap();
        stack.push(String::from("2")).unwrap();
        stack.push(String::from(")")).unwrap();
        let ret_tok: Token = stack.pop_tok().unwrap();
        assert_eq!(ret_tok, Token {lexeme: String::from("(+12)"), my_type: Type::Number})
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
        assert_eq!(ret_tok, Token {lexeme: String::from("(+12345)"), my_type: Type::Number})
    }

    #[test]
    fn pop_empty_stack() {
        let mut stack = TokenStack::init(vec![], 0).unwrap();
        assert_eq!(stack.pop(), Err(ParserErrors::TokenStackEmpty));
        assert_eq!(stack.pop_tok(), Err(ParserErrors::TokenStackEmpty))
    }
}
