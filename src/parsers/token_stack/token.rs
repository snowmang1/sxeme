use crate::parsers::error_matrix::ParserErrors;
use crate::parsers::token_stack::translate_error::TranslationErrors;

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Number,         // i32
    Plus,           // operation
    Minus,
    Divide,
    Multiply,
    Handle,         // static analysis
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    lexeme: String,
    my_type: Type,
}

impl Token {
    pub fn new(lex: String) -> Result<Token, ParserErrors> {
        let mut err: Vec<ParserErrors> = vec![];
        let ret: Option<Token> = match lex.clone().pop().unwrap() {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => Some(Token {
                lexeme: lex,
                my_type: Type::Number,
            }),
            '(' | ')' => Some(Token {
                lexeme: lex,
                my_type: Type::Handle,
            }),
            '+' => Some(Token {
                lexeme: lex,
                my_type: Type::Plus,
            }),
            '-' => Some(Token {
                lexeme: lex,
                my_type: Type::Minus,
            }),
            '/' => Some(Token {
                lexeme: lex,
                my_type: Type::Divide,
            }),
            '*' => Some(Token {
                lexeme: lex,
                my_type: Type::Multiply,
            }),
            _ => {
                err.push(ParserErrors::BadSymbol);
                None
            } // CHANGE
        };
        match ret {
            Some(ok) => Ok(ok),
            _ => Err(err.pop().unwrap()),
        }
    }

    pub fn translate(&self) -> Result<String, TranslationErrors> {
        let mut prog : String = String::from("define i32 @main(){\n");        // llvm requires a main function to exist for an entrypoint
        match &self.my_type {
            Type::Number => unimplemented!(),
            Type::Plus => {
                prog.push_str(r"\%x0 = i32 ");
                for (i, char) in self.lexeme.chars().enumerate() {
                    prog.push_str("\n");
                }
            },
            Type::Minus => unimplemented!(),
            Type::Divide => unimplemented!(),
            Type::Multiply => unimplemented!(),
            _ => panic!("attempt to convert a handle"),
        }
        prog.push('}');
        Err(TranslationErrors::Error)
    }

    pub fn formed(lex: String, output_type: Type) -> Token {
        Token {
            lexeme: lex,
            my_type: output_type,
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
            }
        );
        let new_token = Token::new(String::from("1"));
        assert_eq!(
            new_token.unwrap(),
            Token {
                lexeme: String::from("1"),
                my_type: Type::Number,
            }
        );
        let new_token = Token::new(String::from("2"));
        assert_eq!(
            new_token.unwrap(),
            Token {
                lexeme: String::from("2"),
                my_type: Type::Number,
            }
        );
        let new_token = Token::new(String::from("3"));
        assert_eq!(
            new_token.unwrap(),
            Token {
                lexeme: String::from("3"),
                my_type: Type::Number,
            }
        );
        let new_token = Token::new(String::from("4"));
        assert_eq!(
            new_token.unwrap(),
            Token {
                lexeme: String::from("4"),
                my_type: Type::Number,
            }
        );
        let new_token = Token::new(String::from("5"));
        assert_eq!(
            new_token.unwrap(),
            Token {
                lexeme: String::from("5"),
                my_type: Type::Number,
            }
        );
        let new_token = Token::new(String::from("6"));
        assert_eq!(
            new_token.unwrap(),
            Token {
                lexeme: String::from("6"),
                my_type: Type::Number,
            }
        );
        let new_token = Token::new(String::from("7"));
        assert_eq!(
            new_token.unwrap(),
            Token {
                lexeme: String::from("7"),
                my_type: Type::Number,
            }
        );
        let new_token = Token::new(String::from("8"));
        assert_eq!(
            new_token.unwrap(),
            Token {
                lexeme: String::from("8"),
                my_type: Type::Number,
            }
        );
        let new_token = Token::new(String::from("9"));
        assert_eq!(
            new_token.unwrap(),
            Token {
                lexeme: String::from("9"),
                my_type: Type::Number,
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
                my_type: Type::Plus,
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
                my_type: Type::Minus,
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
                my_type: Type::Divide,
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
                my_type: Type::Multiply,
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
            }
        );
        let new_token = Token::new(String::from(")"));
        assert_eq!(
            new_token.unwrap(),
            Token {
                lexeme: String::from(")"),
                my_type: Type::Handle,
            }
        );
    }

}
