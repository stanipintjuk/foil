named!(pub take_string<String>, map!(delimited!(
        tag!("\""), 
        escaped_transform!(is_not!("\"\\"), '\\', alt!(tag!("\"")|tag!("\\"))), 
        tag!("\"")),
        |bytes| { String::from_utf8(bytes).unwrap() }
    ));

named!(pub take_path<String>, map!(
        delimited!(
            tag!("<"), 
            escaped_transform!(is_not!(">\\"), '\\', alt!(tag!(">")|tag!("\\"))), 
            tag!(">")
        ),
        |bytes| { String::from_utf8(bytes).unwrap() }
        )
    );

named!(pub consume_space, take_while!(is_whitespace));

pub fn is_whitespace(ascii: u8) -> bool {
    ascii == '\n' as u8 || ascii == ' ' as u8 || ascii == '\t' as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::*;

    #[test]
    fn take_path_works() {

        //Should work with regular 'ol paths
        assert_eq!(
            IResult::Done(&b""[..], "/path/to/stuff".to_string()), 
            take_path(b"</path/to/stuff>")
        );

    }

    #[test]
    fn take_path_works_with_spaces() {
        //Should work with spaces without escaping them
        assert_eq!(
            IResult::Done(&b""[..], "/folder name/hello.foil".to_string()),
            take_path(b"</folder name/hello.foil>")
        );
    }

    #[test]
    fn take_path_works_with_relative_paths() {
        //Should work with relative paths
        assert_eq!(
            IResult::Done(&b"to folder"[..], "./path".to_string()),
            take_path(b"<./path>to folder")
        );
    }

    #[test]
    fn take_path_works_with_greater() {
        //Should work with escaped greater-than sign
        assert_eq!(
            IResult::Done(&b"folder"[..], "/path>to".to_string()),
            take_path(b"</path\\>to>folder")
        );
    }

    #[test]
    fn take_path_works_with_backslash() {
        //Should work with escaped backslashes
        assert_eq!(
            IResult::Done(&b"folder"[..], "/path\\to".to_string()),
            take_path(b"</path\\\\to>folder")
        );
    }


    #[test]
    fn take_string_works_normal_case() {
        //Check normal 'ol string
        assert_eq!(
            IResult::Done(&b"not string"[..], "a string".to_string()),
            take_string(b"\"a string\"not string")
        );
    }

    #[test]
    fn take_string_works_with_qoutes() {
        //Should work with escaped qoutes
        assert_eq!(
            IResult::Done(&b"not string"[..], "a \"string\"".to_string()),
            take_string(b"\"a \\\"string\\\"\"not string")
        );
    }

    #[test]
    fn take_string_works_backslash() {
        //Should work with escaped backslash
        assert_eq!(
            IResult::Done(&b"not string"[..], "a \\string".to_string()),
            take_string(b"\"a \\\\string\"not string")
        );
    }
}
