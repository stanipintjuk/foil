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
                self.pos += 2;
                Some(Token::Op(Op::Equals))
        } else {
            self.pos += 1;
            Some(Token::Op(Op::Assign))
        }
    }

    fn lex_mul_or_pow(&mut self) -> Option<Token<'a>> {
        if (self.char_at(self.pos), 
            self.char_at(self.pos + 1)) == (Some('*'), Some('*')) {
                self.pos += 2;
                Some(Token::Op(Op::Pow))
        } else {
            self.pos += 1;
            Some(Token::Op(Op::Mul))
        }
    }

    fn lex_strlit(&mut self) -> Option<Token<'a>> {
        let matched = match_string(&self.buf[self.pos..]);
        if matched == None {
            return None;
        }
        let matched = matched.unwrap();
        let mstr = matched.as_str();
        let mstr = &mstr[1..mstr.len()-1];

        self.pos += matched.end();
        Some(Token::Val(Val::String(mstr)))
    }

    fn lex_pathlit(&mut self) -> Option<Token<'a>> {
        let matched = match_path(&self.buf[self.pos..]);
        if matched == None {
            return None;
        }
        let matched = matched.unwrap();
        let mstr = matched.as_str();
        let mstr = &mstr[1..mstr.len()-1];

        self.pos += matched.end();
        Some(Token::Val(Val::Path(mstr)))
    }

    fn lex_bareword(&mut self) -> Option<Token<'a>> {
        let matched = match_bare_word(&self.buf[self.pos..]);
        if matched == None {
            return None
        }
        let matched =  matched.unwrap();
        let token = match matched.as_str() {
            "let" => Token::Keyword(Keyword::Let),
            "fn" => Token::Keyword(Keyword::Fn),
            "import" => Token::Keyword(Keyword::Import),
            "set" => Token::Keyword(Keyword::Set),
            x => Token::Id(x),
        };
        self.pos += matched.end();
        Some(token)
        
    }

    fn lex_numlit(&mut self) -> Option<Token<'a>> {
        if let Some(mdouble) = match_double(&self.buf[self.pos..]) {
            let d = mdouble.as_str().parse::<f64>().unwrap();
            self.pos += mdouble.end();
            Some(Token::Val(Val::Double(d)))

        } else if let Some(mint) = match_int(&self.buf[self.pos..]) {
            let i = mint.as_str().parse::<i64>().unwrap();
            self.pos += mint.end();
            Some(Token::Val(Val::Int(i)))

        } else {
            None
        }
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
                self.pos += 1;
                Some(Token::Op(Op::Add))
            },
            '-' => {
                self.pos += 1;
                Some(Token::Op(Op::Sub))
            },
            '/' => {
                self.pos += 1;
                Some(Token::Op(Op::Div))
            },
            '%' => {
                self.pos += 1;
                Some(Token::Op(Op::Mod))
            },
            '!' => {
                self.pos += 1;
                Some(Token::Op(Op::Not))
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
            Token::Op(Op::Sub),
            Token::Op(Op::Mul),
            Token::Op(Op::Div),
            Token::Op(Op::Mod),
            Token::Op(Op::Pow),
            Token::Op(Op::Add),
            Token::Op(Op::Equals),
            Token::Op(Op::Assign),
            Token::Op(Op::Not),
        ];

        let actual: Vec<Token> = Tokenizer::new(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_tokenizer_native_vals_work() {
        let input = "123 123.123 \"string\" <path>";
        
        let expected = vec![
            Token::Val(Val::Int(123)),
            Token::Val(Val::Double(123.123)),
            Token::Val(Val::String("string")),
            Token::Val(Val::Path("path")),
        ];

        let actual: Vec<Token> = Tokenizer::new(input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_tokenizer_keywords_work() {
        let input = "let fn import set";
        
        let expected = vec![
            Token::Keyword(Keyword::Let),
            Token::Keyword(Keyword::Fn),
            Token::Keyword(Keyword::Import),
            Token::Keyword(Keyword::Set),
        ];

        let actual: Vec<Token> = Tokenizer::new(input).collect();
        assert_eq!(expected, actual);
    }
}
