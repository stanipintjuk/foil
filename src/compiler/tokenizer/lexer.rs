enum LexError<'a> {
    Garbage(&'a str, usize, usize);
}

pub fn lex<'a>(text: &'a str) -> Result<Vec<Token<'a>>, LexError> {

}
