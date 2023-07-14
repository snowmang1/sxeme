use crate::parsers::error_matrix::ParserErrors;
use crate::parsers::token_stack::translate_error::TranslationErrors;

#[derive(Debug, PartialEq, Clone)]
pub enum Operation {
    Plus,           // operation
    Minus,
    Divide,
    Multiply,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Number,         // i32
    Handle,         // static analysis
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    lexeme: String,
    my_type: Type,
    op: Option<Operation>,
}

impl Token {
    pub fn new(lex: String) -> Result<Token, ParserErrors> {
        let mut err: Vec<ParserErrors> = vec![];
        let ret: Option<Token> = match lex.clone().pop().unwrap() {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => Some(Token {
                lexeme: lex,
                my_type: Type::Number,
                op: None,
            }),
            '(' => {
                if lex.len() <= 1 {
                    Some(Token{lexeme: lex, my_type: Type::Handle, op: None})
                }
                else { // itterate to verify operation presence
                    for (i, c) in lex.chars().enumerate() {
                        match c {
                            
                        }
                    }
                }
            },
                /* The correct typing is not obvious here lex should be iterated over in order to
                 * assure validity. 
                 * This also means a separation of ( & )
                */
            ')' => Some(Token {
                lexeme: lex,
                my_type: Type::Handle,
                op: None,
            }),
            '+' => Some(Token {
                lexeme: lex,
                my_type: Type::Number,
                op: Some(Operation::Plus),
            }),
            '-' => Some(Token {
                lexeme: lex,
                my_type: Type::Number,
                op: Some(Operation::Minus),
            }),
            '/' => Some(Token {
                lexeme: lex,
                my_type: Type::Number,
                op: Some(Operation::Divide),
            }),
            '*' => Some(Token {
                lexeme: lex,
                my_type: Type::Number,
                op:  Some(Operation::Multiply),
            }),
            _ => {
                err.push(ParserErrors::BadSymbol);
                None
            },
        };
        match ret {
            Some(ok) => Ok(ok),
            _ => Err(err.pop().unwrap()),
        }
    }

    pub fn translate(&self, temp_mangle: &mut (String, i32)) -> Result<String, TranslationErrors> {
        let mut prog : String = String::from("define i32 @main(){\n");        // llvm requires a main function to exist for an entrypoint
        match self.op {
            Some(Operation::Plus) => {
                for (i, char) in self.lexeme.chars().enumerate() {
                    match char {
                        x if (x=='1' || x=='2' || x=='3' || x=='4' || x=='5' || x=='6' || x=='7' || x=='8' || x=='9' || x=='0') && i!=0 => break,
                        x if (x=='1' || x=='2' || x=='3' || x=='4' || x=='5' || x=='6' || x=='7' || x=='8' || x=='9' || x=='0') && i==0 => {
                            let mut line = String::from('%');
                            line.push_str(&temp_mangle.0.clone());
                            line.push_str(&temp_mangle.1.to_string().clone());
                            temp_mangle.1 += 1;
                            line.push_str("= i32 ");
                            line.push(x);
                            line.push('\n');
                        },
                        _ => (),
                    }
                }
            },
            Some(Operation::Minus) => unimplemented!(),
            Some(Operation::Divide) => unimplemented!(),
            Some(Operation::Multiply) => unimplemented!(),
            _ => (),
        }
        prog.push('}');
        Ok(prog)
    }

    pub fn formed(lex: String, output_type: Type) -> Token {
        Token {
            lexeme: lex,
            my_type: output_type,
            op: None,
        }
    }
    pub fn as_str(&self) -> &str {
        self.lexeme.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_number() {
        let new_token = Token::new(String::from("0"));
        assert_eq!(
            new_token.unwrap(),
            Token {
                lexeme: String::from("0"),
                my_type: Type::Number,
                op: None,
            }
        );
        let new_token = Token::new(String::from("1"));
        assert_eq!(
            new_token.unwrap(),
            Token {
                lexeme: String::from("1"),
                my_type: Type::Number,
                op: None,
            }
        );
        let new_token = Token::new(String::from("2"));
        assert_eq!(
            new_token.unwrap(),
            Token {
                lexeme: String::from("2"),
                my_type: Type::Number,
                op: None,
            }
        );
        let new_token = Token::new(String::from("3"));
        assert_eq!(
            new_token.unwrap(),
            Token {
                lexeme: String::from("3"),
                my_type: Type::Number,
                op: None,
            }
        );
        let new_token = Token::new(String::from("4"));
        assert_eq!(
            new_token.unwrap(),
            Token {
                lexeme: String::from("4"),
                my_type: Type::Number,
                op: None,
            }
        );
        let new_token = Token::new(String::from("5"));
        assert_eq!(
            new_token.unwrap(),
            Token {
                lexeme: String::from("5"),
                my_type: Type::Number,
                op: None,
            }
        );
        let new_token = Token::new(String::from("6"));
        assert_eq!(
            new_token.unwrap(),
            Token {
                lexeme: String::from("6"),
                my_type: Type::Number,
                op: None,
            }
        );
        let new_token = Token::new(String::from("7"));
        assert_eq!(
            new_token.unwrap(),
            Token {
                lexeme: String::from("7"),
                my_type: Type::Number,
                op: None,
            }
        );
        let new_token = Token::new(String::from("8"));
        assert_eq!(
            new_token.unwrap(),
            Token {
                lexeme: String::from("8"),
                my_type: Type::Number,
                op: None,
            }
        );
        let new_token = Token::new(String::from("9"));
        assert_eq!(
            new_token.unwrap(),
            Token {
                lexeme: String::from("9"),
                my_type: Type::Number,
                op: None,
            }
        );
    }

    #[test]
    fn type_plus() {
        let new_token = Token::new(String::from("+"));
        assert_eq!(
            new_token.unwrap(),
            Token {
                lexeme: String::from("+"),
                my_type: Type::Number,
                op: Some(Operation::Plus),
            }
        );
    }

    #[test]
    fn type_minus() {
        let new_token = Token::new(String::from("-"));
        assert_eq!(
            new_token.unwrap(),
            Token {
                lexeme: String::from("-"),
                my_type: Type::Number,
                op: Some(Operation::Minus),
            }
        );
    }

    #[test]
    fn type_divide() {
        let new_token = Token::new(String::from("/"));
        assert_eq!(
            new_token.unwrap(),
            Token {
                lexeme: String::from("/"),
                my_type: Type::Number,
                op: Some(Operation::Divide),
            }
        );
    }

    #[test]
    fn type_multiply() {
        let new_token = Token::new(String::from("*"));
        assert_eq!(
            new_token.unwrap(),
            Token {
                lexeme: String::from("*"),
                my_type: Type::Number,
                op: Some(Operation::Multiply),
            }
        );
    }

    #[test]
    fn type_handle() {
        let new_token = Token::new(String::from("("));
        assert_eq!(
            new_token.unwrap(),
            Token {
                lexeme: String::from("("),
                my_type: Type::Handle,
                op: None,
            }
        );
        let new_token = Token::new(String::from(")"));
        assert_eq!(
            new_token.unwrap(),
            Token {
                lexeme: String::from(")"),
                my_type: Type::Handle,
                op: None,
            }
        );
    }

}
