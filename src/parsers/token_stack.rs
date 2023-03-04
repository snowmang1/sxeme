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
        let mut ret: Option<Token> = None;
        for c in lex.chars() { // find operator and decide based on operators definition
            ret = match c {
                '0' | '1' | '2' | '3' | '4' | '5' |
                    '6' | '7' | '8' | '9' => Some (Token { lexeme: lex, my_type: Type::Number}),
                '(' | ')' => Some (Token { lexeme: lex, my_type: Type::Handle}),
                '+' => Some (Token { lexeme: lex, my_type: Type::Plus }),
                '-' => Some (Token { lexeme: lex, my_type: Type::Minus }),
                '/' => Some (Token { lexeme: lex, my_type: Type::Divide }),
                '*' => Some (Token { lexeme: lex, my_type: Type::Multiply }),
                _ => continue
            };
            break;
        }
        match ret { Some(ok) => Ok(ok), _ => Err(ParserErrors::BadSymbol) }
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
    fn init(st: Vec<String>, delim_count: u32) -> Result<Self, ParserErrors> {
        let mut stack: Vec<Token> = vec![];
        for lexeme in st {
            stack.push(Token::new(lexeme)?);
        }
        Ok(TokenStack { stack, delim_count })
    }
    // above is DEBUG use only
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
                        "(" => { accum.insert(0, '('); self.stack.pop(); self.stack.push(Token::new(accum)?); break },
                        ind => { accum.insert_str(0, ind); self.stack.pop(); }
                    }
                }
                if self.delim_count >= 1 { self.delim_count-=1; }
                match err {
                    ParserErrors::Good => Ok(()),
                    e => Err(e)
                }
            }
            "+" => { // state 2 operation
                self.stack.push(Token::new(lex)?);
                Ok(())
            }
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => { // state 3
                self.stack.push(Token::new(lex)?);
                Ok(())
            }
            _   => Err(ParserErrors::UnknownSymbol(lex))
        }
    }

    pub fn pop(&mut self) -> Result<String, ParserErrors> {
        match self.stack.pop() { None => Err(ParserErrors::TokenStackEmpty), Some(s) => Ok(String::from(s.as_str())) }
    }
    pub fn len(&self) -> usize {
        self.stack.len()
    }
}

