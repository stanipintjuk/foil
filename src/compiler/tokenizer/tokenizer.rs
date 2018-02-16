use super::tokens::*;
use super::regex::*;

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Tokenizer<'a> {
    buf: &'a str,
    pos: usize,
}
impl<'a> Tokenizer<'a> {
    fn new(buf: &'a str) -> Tokenizer<'a> {
        Tokenizer{ buf: buf, pos: 0 }
    }

    fn char_at(&self, pos: usize) -> Option<char> {
        self.buf.chars().nth(pos)
    }
    fn lex_assign_or_equals(&mut self) -> Option<Token<'a>> {
        if (self.char_at(self.pos), 
            self.char_at(self.pos + 1)) == (Some('='), Some('=')) {
                let result = Some(Token::BinOp(self.pos, BinOp::Equals));
                self.pos += 2;
                return result;
        } else {
            let result = Some(Token::BinOp(self.pos, BinOp::Assign));
            self.pos += 1;
            return result;
        }
    }

    fn lex_mul_or_pow(&mut self) -> Option<Token<'a>> {
        if (self.char_at(self.pos), 
            self.char_at(self.pos + 1)) == (Some('*'), Some('*')) {
                let result = Some(Token::BinOp(self.pos, BinOp::Pow));
                self.pos += 2;
                return result;
        } else {
            let result = Some(Token::BinOp(self.pos, BinOp::Mul));
            self.pos += 1;
            return result;
        }
    }

    fn lex_strlit(&mut self) -> Option<Token<'a>> {
        let matched = match_string(&self.buf[self.pos..]);
        if matched == None {
            return Some(self.garbage());
        }
        let matched = matched.unwrap();
        let mstr = matched.as_str();
        let mstr = &mstr[1..mstr.len()-1];

        let result = Some(Token::Val(self.pos, Val::String(mstr)));
        self.pos += matched.end();
        return result;
    }

    fn lex_pathlit(&mut self) -> Option<Token<'a>> {
        let matched = match_path(&self.buf[self.pos..]);
        if matched == None {
            return Some(self.garbage());
        }
        let matched = matched.unwrap();
        let mstr = matched.as_str();
        let mstr = &mstr[1..mstr.len()-1];

        let result = Some(Token::Val(self.pos, Val::Path(mstr)));
        self.pos += matched.end();
        return result;
    }

    fn lex_bareword(&mut self) -> Option<Token<'a>> {
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
            x => Token::Id(self.pos, x),
        };
        self.pos += matched.end();
        Some(token)
        
    }

    fn lex_numlit(&mut self) -> Option<Token<'a>> {
        if let Some(mdouble) = match_double(&self.buf[self.pos..]) {
            let d = mdouble.as_str().parse::<f64>().unwrap();
            let result = Some(Token::Val(self.pos, Val::Double(d)));
            self.pos += mdouble.end();
            return result;

        } else if let Some(mint) = match_int(&self.buf[self.pos..]) {
            let i = mint.as_str().parse::<i64>().unwrap();
            let result = Some(Token::Val(self.pos, Val::Int(i)));
            self.pos += mint.end();
            return result;
        } else {
            Some(self.garbage())
        }
    }
    fn garbage(&mut self) -> Token<'a> {
        let token = Token::Garbage(self.pos, &self.buf[self.pos..]);
        self.pos = self.buf.len();
        token
    }
}
impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token<'a>;
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
            '+' => {
                let res = Some(Token::BinOp(self.pos, BinOp::Add));
                self.pos += 1;
                return res;
            },
            '-' => {
                let res = Some(Token::BinOp(self.pos, BinOp::Sub));
                self.pos += 1;
                return res;
            },
            '/' => {
                let res = Some(Token::BinOp(self.pos, BinOp::Div));
                self.pos += 1;
                return res;
            },
            '%' => {
                let res = Some(Token::BinOp(self.pos, BinOp::Mod));
                self.pos += 1;
                return res;
            },
            '!' => {
                let res = Some(Token::UnaryOp(self.pos, UnaryOp::Not));
                self.pos += 1;
                return res;
            },
            '=' => self.lex_assign_or_equals(),
            '*' => self.lex_mul_or_pow(),
            '"' => self.lex_strlit(),
            '<' => self.lex_pathlit(),
            x if x.is_alphabetic() => self.lex_bareword(),
            x if x.is_numeric() => self.lex_numlit(),
            _ => None,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use super::super::tokens::*;

    #[test]
    fn test_tokenizer_native_ops_work() {
        let input = "- * / % ** + == = !";

        let expected = vec![
            Token::BinOp(0, BinOp::Sub),
            Token::BinOp(2, BinOp::Mul),
            Token::BinOp(4, BinOp::Div),
            Token::BinOp(6, BinOp::Mod),
            Token::BinOp(8, BinOp::Pow),
            Token::BinOp(11, BinOp::Add),
            Token::BinOp(13, BinOp::Equals),
            Token::BinOp(16, BinOp::Assign),
            Token::UnaryOp(18, UnaryOp::Not),
        ];

        let actual: Vec<Token> = Tokenizer::new(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_tokenizer_native_vals_work() {
        let input = "123 123.123 \"string\" <path>";
        
        let expected = vec![
            Token::Val(0, Val::Int(123)),
            Token::Val(4, Val::Double(123.123)),
            Token::Val(12, Val::String("string")),
            Token::Val(21, Val::Path("path")),
        ];

        let actual: Vec<Token> = Tokenizer::new(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_tokenizer_keywords_work() {
        let input = "let fn import set";
        
        let expected = vec![
            Token::Keyword(0, Keyword::Let),
            Token::Keyword(4, Keyword::Fn),
            Token::Keyword(7, Keyword::Import),
            Token::Keyword(14, Keyword::Set),
        ];

        let actual: Vec<Token> = Tokenizer::new(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_garbage_string() {
        let input = "\"test ";
        let expected = vec![ Token::Garbage(0, "\"test ") ];
        let actual: Vec<Token> = Tokenizer::new(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_garbage_path() {
        let input = "<path ";
        let expected = vec![ Token::Garbage(0, "<path ") ];
        let actual: Vec<Token> = Tokenizer::new(input).collect();
        assert_eq!(expected, actual);
    }
}
