#[derive(Debug, PartialEq, Clone)]
pub enum ParserErrors {
    TokenStackEmpty, // a pop occured while token stack was empty
    BadSymbol,       // value does not belong in definition
    Good,            // value representing no error CHANGE
    NoClosingParen,  // Occurs when a paren block is never closed
    NoOpeningParen,  // Occurs when a paren block is never opened
                     //UnknownSymbol(String),  // Occurs when the symbol is not expected
}
