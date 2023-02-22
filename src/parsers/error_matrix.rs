
#[allow(dead_code)] //TODO get rid of this
#[derive(Debug, PartialEq, Clone)]
pub enum ParserErrors {
    Good,                   // value representing no error
    NoClosingParen,         // Occurs when a paren block is never closed
    NoOpeningParen,         // Occurs when a paren block is never opened
    UnknownSymbol(String),  // Occurs when the symbol is not expected
}

/* TODO
 * - NoClosingParen must happen inside top level parser as nothing else has
 * the concept "end of stream"
 */
