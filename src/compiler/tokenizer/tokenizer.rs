use compiler::models::{Token, BinOp, UnaryOp, Val, Keyword};
use super::regex::{
    end_of_whitespace,
    match_bare_word,
    match_double,
    match_int,
    match_string,
    match_path
};
use compiler::errors::TokenError;

pub type TokenResult = Result<Token, TokenError>;
pub type TokenIterator<'i> = Iterator<Item=TokenResult> + 'i;

// Convinient way to return a token and move the lexer position forward.
// Example:
// token!(Token::BinOp => BinOp::Add, lexer=>1)
// Means "Create a token of type BinOp and of variant 'Add', and then move the lexer pointer 1 step"
macro_rules! token {
    ( $token:expr => $sub_token:expr, $lexer:expr => $size:expr) => {{
        let result = Some(Ok($token($lexer.pos, $sub_token)));
        $lexer.pos += $size;
        return result;
    }};

    ( $token:expr, $lexer:expr => $size:expr ) => {{
        let result = Some(Ok($token($lexer.pos)));
        $lexer.pos += $size;
        return result;
    }};
}

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Tokenizer<'a> {
    buf: &'a str,
    pos: usize,
}
impl<'a> Tokenizer<'a> {
    pub fn new(buf: &'a str) -> Tokenizer<'a> {
        Tokenizer{ buf: buf, pos: 0 }
    }

    fn char_at(&self, pos: usize) -> Option<char> {
        self.buf.chars().nth(pos)
    }
    fn lex_assign_or_equals(&mut self) -> Option<TokenResult> {
        if (self.char_at(self.pos), self.char_at(self.pos + 1)) == (Some('='), Some('=')) {
            token!(Token::BinOp => BinOp::Equals, self => 2)
        } else {
            token!(Token::Assign, self => 1)
        }
    }

    fn lex_mul_or_pow(&mut self) -> Option<TokenResult> {
        if (self.char_at(self.pos), self.char_at(self.pos + 1)) == (Some('*'), Some('*')) {
            token!(Token::BinOp => BinOp::Pow, self => 2)
        } else {
            token!(Token::BinOp => BinOp::Mul, self => 1)
        }
    }

    fn lex_strlit(&mut self) -> Option<TokenResult> {
        let matched = match_string(&self.buf[self.pos..]);
        if matched == None {
            return Some(self.garbage());
        }
        let matched = matched.unwrap();
        let mstr = matched.as_str();
        let mstr = &mstr[1..mstr.len()-1];

        token!(Token::Val => Val::String(mstr.to_string()), self => matched.end())
    }

    fn lex_pathlit(&mut self) -> Option<TokenResult> {
        let matched = match_path(&self.buf[self.pos..]);
        if matched == None {
            return Some(self.garbage());
        }
        let matched = matched.unwrap();
        let mstr = matched.as_str();
        let mstr = &mstr[1..mstr.len()-1];

        token!(Token::Val => Val::Path(mstr.to_string()), self => matched.end())
    }

    fn lex_bareword(&mut self) -> Option<TokenResult> {
        let matched = match_bare_word(&self.buf[self.pos..]);
        if matched == None {
            return Some(self.garbage());
        }
        let matched =  matched.unwrap();
        let token = match matched.as_str() {
            "let" => Token::Keyword(self.pos, Keyword::Let),
            "fn" => Token::Keyword(self.pos, Keyword::Fn),
            "import" => Token::Keyword(self.pos, Keyword::Import),
            "set" => Token::Keyword(self.pos, Keyword::Set),
            "in" => Token::Keyword(self.pos, Keyword::In),
            "true" => Token::Val(self.pos, Val::Bool(true)),
            "false" => Token::Val(self.pos, Val::Bool(false)),
            x => Token::Id(self.pos, x.to_string()),
        };
        self.pos += matched.end();
        Some(Ok(token))
    }

    fn lex_numlit(&mut self) -> Option<TokenResult> {
        if let Some(mdouble) = match_double(&self.buf[self.pos..]) {
            let d = mdouble.as_str().parse::<f64>().unwrap();
            token!(Token::Val => Val::Double(d), self => mdouble.end())

        } else if let Some(mint) = match_int(&self.buf[self.pos..]) {
            let i = mint.as_str().parse::<i64>().unwrap();
            token!(Token::Val => Val::Int(i), self => mint.end())

        } else {
            Some(self.garbage())
        }
    }

    fn is_html_keyword(&self) -> bool {
        if self.pos+5 > self.buf.len() {
            false
        } else if &self.buf[self.pos..self.pos+5] == "html!" {
            true
        } else {
            false
        }
    }

    fn garbage(&mut self) -> TokenResult {
        let err = TokenError::Garbage(self.pos, self.buf[self.pos..].to_string());
        self.pos = self.buf.len();
        Err(err)
    }
}
impl<'a> Iterator for Tokenizer<'a> {
    type Item = TokenResult;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(end) = end_of_whitespace(&self.buf[self.pos..]) {
                self.pos += end;
        }
        if self.pos >= self.buf.len() {
            return None;
        }
        let current_char = self.char_at(self.pos);
        if current_char == None {
            return None;
        }
        let current_char = current_char.unwrap();
        match current_char {
            '+' => token!(Token::BinOp => BinOp::Add, self=>1),
            '-' => token!(Token::BinOp => BinOp::Sub, self=>1),
            '/' => token!(Token::BinOp => BinOp::Div, self=>1),
            '%' => token!(Token::BinOp => BinOp::Mod, self=>1),
            '!' => token!(Token::UnaryOp => UnaryOp::Not, self=>1),
            '(' => token!(Token::GroupL, self=>1),
            ')' => token!(Token::GroupR, self=>1),
            '{' => token!(Token::BlockL, self=>1),
            '}' => token!(Token::BlockR, self=>1),
            ',' => token!(Token::Comma, self=>1),
            ':' => token!(Token::Colon, self=>1),
            ';' => token!(Token::Semi, self=>1),
            '=' => self.lex_assign_or_equals(),
            '*' => self.lex_mul_or_pow(),
            '"' => self.lex_strlit(),
            '<' => self.lex_pathlit(),
            'h' if self.is_html_keyword() => token!(Token::Keyword => Keyword::Html, self=>5),
            x if x.is_alphabetic() => self.lex_bareword(),
            x if x.is_numeric() => self.lex_numlit(),
            _ => Some(self.garbage()),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use compiler::models::tokens::*;

    #[test]
    fn test_tokenizer_native_ops_work() {
        let input = "- * / % ** + == = !";

        let expected = vec![
            Ok(Token::BinOp(0, BinOp::Sub)),
            Ok(Token::BinOp(2, BinOp::Mul)),
            Ok(Token::BinOp(4, BinOp::Div)),
            Ok(Token::BinOp(6, BinOp::Mod)),
            Ok(Token::BinOp(8, BinOp::Pow)),
            Ok(Token::BinOp(11, BinOp::Add)),
            Ok(Token::BinOp(13, BinOp::Equals)),
            Ok(Token::Assign(16)),
            Ok(Token::UnaryOp(18, UnaryOp::Not)),
        ];

        let actual: Vec<_> = Tokenizer::new(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_tokenizer_native_vals_work() {
        let input = "123 123.123 \"string\" <path>";
        
        let expected = vec![
            Ok(Token::Val(0, Val::Int(123))),
            Ok(Token::Val(4, Val::Double(123.123))),
            Ok(Token::Val(12, Val::String("string".to_string()))),
            Ok(Token::Val(21, Val::Path("path".to_string()))),
        ];

        let actual: Vec<_> = Tokenizer::new(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_tokenizer_keywords_work() {
        let input = "let fn import set in";
        
        let expected = vec![
            Ok(Token::Keyword(0, Keyword::Let)),
            Ok(Token::Keyword(4, Keyword::Fn)),
            Ok(Token::Keyword(7, Keyword::Import)),
            Ok(Token::Keyword(14, Keyword::Set)),
            Ok(Token::Keyword(18, Keyword::In)),
        ];

        let actual: Vec<_> = Tokenizer::new(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_garbage_string() {
        let input = "\"test ";
        let expected = vec![ Err(TokenError::Garbage(0, "\"test ".to_string())) ];
        let actual: Vec<_> = Tokenizer::new(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_garbage_path() {
        let input = "<path ";
        let expected = vec![ Err(TokenError::Garbage(0, "<path ".to_string())) ];
        let actual: Vec<_> = Tokenizer::new(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parens() {
        let input = "( ( 12 ))";
        let expected = vec![
            Ok(Token::GroupL(0)),
            Ok(Token::GroupL(2)),
            Ok(Token::Val(4, Val::Int(12))),
            Ok(Token::GroupR(7)),
            Ok(Token::GroupR(8)),
        ];

        let actual: Vec<_> = Tokenizer::new(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_block_parens() {
        let input = "{ { 12 }}";
        let expected = vec![
            Ok(Token::BlockL(0)),
            Ok(Token::BlockL(2)),
            Ok(Token::Val(4, Val::Int(12))),
            Ok(Token::BlockR(7)),
            Ok(Token::BlockR(8)),
        ];

        let actual: Vec<_> = Tokenizer::new(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_punctuation() {
        let input = ", : ;";
        let expected = vec![
            Ok(Token::Comma(0)),
            Ok(Token::Colon(2)),
            Ok(Token::Semi(4)),
        ];

        let actual: Vec<_> = Tokenizer::new(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_tokenizing_html_works() {
        let input = "html! html{ h1 \"test\" }";
        let expected = vec![
            Ok(Token::Keyword(0, Keyword::Html)),
            Ok(Token::Id(6, "html".to_string())),
            Ok(Token::BlockL(10)),
            Ok(Token::Id(12, "h1".to_string())),
            Ok(Token::Val(15, Val::String("test".to_string()))),
            Ok(Token::BlockR(22))
        ];

        let actual: Vec<_> = Tokenizer::new(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn should_not_panic_if_html_keyword_is_not_complete() {
        // the `html!` keyword is a special case for this tokenizer and might panic if the keyword
        // starts but doesn't end after 5 characters.
        
        let input = "h";
        let expected = vec![
            Ok(Token::Id(0, "h".to_string())),
        ];
        let actual: Vec<_> = Tokenizer::new(input).collect();
        assert_eq!(expected, actual);
    }
}
