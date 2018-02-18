/// Convinient macros for the parser

macro_rules! expect_expression {
    ( $parser:expr, $pos:expr ) => {
        match $parser.next() {
            Some(Ok(expr)) => expr,
            Some(Err(err)) => {
                return Some(Err(err));
            }
            None => {
                return Some(Err(ParseError::ExpectedExpression($pos)));
            }
        }
    }
}

macro_rules! next_token {
    ( $lexer:expr, $pos:expr ) => {
        match $lexer.next() {
            Some(Ok(token)) => token,
            Some(Err(err)) => {
                return Some(Err(ParseError::Lexer(err)));
            }
            None => {
                return Some(Err(ParseError::UnexpectedEndOfCode($pos)));
            }
        }
    }
}

macro_rules! expect_assignment {
    ( $lexer:expr, $pos:expr ) => {{
        let token = next_token!($lexer, $pos);
        let pos = match token {
            Token::BinOp(pos, BinOp::Assign) => { pos },
            token => {
                return Some(Err(ParseError::ExpectedAssignment(token)));
            }
        };
        pos
    }}
}

macro_rules! expect_keyword {
    ($keyword:expr, $lexer:expr, $pos:expr) => {{
        let token = next_token!($lexer, $pos);
        let pos = match token {
            Token::Keyword(pos, keyword) => { 
                if keyword == $keyword {
                    pos 
                } else {
                    return Some(Err(
                            ParseError::ExpectedKeyword($keyword, 
                                                        Token::Keyword(pos, keyword))));
                }
            },
            token => {
                return Some(Err(ParseError::ExpectedKeyword($keyword, token)));
            }
        };
        pos
    }}
}

macro_rules! expect_id {
    ($lexer:expr, $pos:expr) => {{
        let token = next_token!($lexer, $pos);
        match token {
            Token::Id(pos, name) => (pos, name),
            token => { 
                return Some(Err(ParseError::ExpectedId(token)));
            }
        }
    }}
}

macro_rules! expect_group_r {
    ($lexer:expr, $pos:expr) => {{
        let token = next_token!($lexer, $pos);
        match token {
            Token::GroupR(pos) => pos,
            token => { 
                return Some(Err(ParseError::ExpectedGroupR(token)));
            }
        }
    }}
}


