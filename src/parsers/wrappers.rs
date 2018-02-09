use parsers::dom::parsers::parse_dom_tree;
use parsers::dom::types::{DOMTree};
use parsers::errors;
use nom::{Needed, IResult, Err as NomErr, ErrorKind as NomErrorKind};
use std::vec::Vec;
use std::iter::Iterator;

#[derive(Debug)]
pub enum FoilCustomError {
    InvalidTagName,
    InvalidAttributeName,
    AttributeValueNotString,
    InvalidExpression,
    InvalidDOMNode,
    InvalidSelfClosingDOMNode,
    ExpectingDelimiters,
}

type FoilErrorKind = NomErrorKind<FoilCustomError>;

fn error_code_to_error_kind(error_code: u32) -> FoilCustomError {
    match error_code {
        errors::ERROR_INVALID_TAG_NAME => FoilCustomError::InvalidTagName,
        errors::ERROR_INVALID_ATTRIBUTE_NAME => FoilCustomError::InvalidAttributeName,
        errors::ERROR_ATTRIBUTE_VALUE_NOT_STRING => FoilCustomError::AttributeValueNotString,
        errors::ERROR_INVALID_EXPRESSION => FoilCustomError::InvalidExpression,
        errors::ERROR_INVALID_DOM_NODE => FoilCustomError::InvalidDOMNode,
        errors::ERROR_INVALID_SELF_CLOSING_DOM_NODE => FoilCustomError::InvalidSelfClosingDOMNode,
        errors::ERROR_EXPECTING_DELIMITERS => FoilCustomError::ExpectingDelimiters,
        _ => { panic!("Error code '{}' is unknown.", error_code); }
    }
}

fn to_foil_error_kind(err: &NomErrorKind<u32>) -> NomErrorKind<FoilCustomError> {
    match err {
        &NomErrorKind::Custom(ref err_code) 
            => NomErrorKind::Custom(error_code_to_error_kind(*err_code)),
        &NomErrorKind::Tag => NomErrorKind::Tag,
        &NomErrorKind::MapRes => NomErrorKind::MapRes,
        &NomErrorKind::MapOpt => NomErrorKind::MapOpt,
        &NomErrorKind::Alt => NomErrorKind::Alt,
        &NomErrorKind::IsNot => NomErrorKind::IsNot,
        &NomErrorKind::IsA => NomErrorKind::IsA,
        &NomErrorKind::SeparatedList => NomErrorKind::SeparatedList,
        &NomErrorKind::SeparatedNonEmptyList => NomErrorKind::SeparatedNonEmptyList,
        &NomErrorKind::Many0 => NomErrorKind::Many0,
        &NomErrorKind::Many1 => NomErrorKind::Many1,
        &NomErrorKind::ManyTill => NomErrorKind::ManyTill,
        &NomErrorKind::Count => NomErrorKind::Count,
        &NomErrorKind::TakeUntilAndConsume => NomErrorKind::TakeUntilAndConsume,
        &NomErrorKind::TakeUntil => NomErrorKind::TakeUntil,
        &NomErrorKind::TakeUntilEitherAndConsume => NomErrorKind::TakeUntilEitherAndConsume,
        &NomErrorKind::TakeUntilEither => NomErrorKind::TakeUntilEither,
        &NomErrorKind::LengthValue => NomErrorKind::LengthValue,
        &NomErrorKind::TagClosure => NomErrorKind::TagClosure,
        &NomErrorKind::Alpha => NomErrorKind::Alpha,
        &NomErrorKind::Digit => NomErrorKind::Digit,
        &NomErrorKind::HexDigit => NomErrorKind::HexDigit,
        &NomErrorKind::OctDigit => NomErrorKind::OctDigit,
        &NomErrorKind::AlphaNumeric => NomErrorKind::AlphaNumeric,
        &NomErrorKind::Space => NomErrorKind::Space,
        &NomErrorKind::MultiSpace => NomErrorKind::MultiSpace,
        &NomErrorKind::LengthValueFn => NomErrorKind::LengthValueFn,
        &NomErrorKind::Eof => NomErrorKind::Eof,
        &NomErrorKind::ExprOpt => NomErrorKind::ExprOpt,
        &NomErrorKind::ExprRes => NomErrorKind::ExprRes,
        &NomErrorKind::CondReduce => NomErrorKind::CondReduce,
        &NomErrorKind::Switch => NomErrorKind::Switch,
        &NomErrorKind::TagBits => NomErrorKind::TagBits,
        &NomErrorKind::OneOf => NomErrorKind::OneOf,
        &NomErrorKind::NoneOf => NomErrorKind::NoneOf,
        &NomErrorKind::Char => NomErrorKind::Char,
        &NomErrorKind::CrLf => NomErrorKind::CrLf,
        &NomErrorKind::RegexpMatch => NomErrorKind::RegexpMatch,
        &NomErrorKind::RegexpMatches => NomErrorKind::RegexpMatches,
        &NomErrorKind::RegexpFind => NomErrorKind::RegexpFind,
        &NomErrorKind::RegexpCapture => NomErrorKind::RegexpCapture,
        &NomErrorKind::RegexpCaptures => NomErrorKind::RegexpCaptures,
        &NomErrorKind::TakeWhile1 => NomErrorKind::TakeWhile1,
        &NomErrorKind::Complete => NomErrorKind::Complete,
        &NomErrorKind::Fix => NomErrorKind::Fix,
        &NomErrorKind::Escaped => NomErrorKind::Escaped,
        &NomErrorKind::EscapedTransform => NomErrorKind::EscapedTransform,
        &NomErrorKind::TagStr => NomErrorKind::TagStr,
        &NomErrorKind::IsNotStr => NomErrorKind::IsNotStr,
        &NomErrorKind::IsAStr => NomErrorKind::IsAStr,
        &NomErrorKind::TakeWhile1Str => NomErrorKind::TakeWhile1Str,
        &NomErrorKind::NonEmpty => NomErrorKind::NonEmpty,
        &NomErrorKind::ManyMN => NomErrorKind::ManyMN,
        &NomErrorKind::TakeUntilAndConsumeStr => NomErrorKind::TakeUntilAndConsumeStr,
        &NomErrorKind::TakeUntilStr => NomErrorKind::TakeUntilStr,
        &NomErrorKind::Not => NomErrorKind::Not,
        &NomErrorKind::Permutation => NomErrorKind::Permutation,
        &NomErrorKind::Verify => NomErrorKind::Verify,
        &NomErrorKind::TakeTill1 => NomErrorKind::TakeTill1,
    }
}

#[derive(Debug)]
pub enum FoilErr {
    Code(FoilErrorKind),
    Node(FoilErrorKind, Vec<FoilErr>),
    Position(FoilErrorKind, usize),
    NodePosition(FoilErrorKind, usize, Vec<FoilErr>),
    Incomplete(Needed),
}

pub fn foil_parser(buf: &[u8]) -> Result<DOMTree, FoilErr> {
    let parse_result = parse_dom_tree(buf);
    match parse_result {
        IResult::Done(_, result) => Ok(result),
        IResult::Incomplete(needed) => Err(FoilErr::Incomplete(needed)),
        IResult::Error(err) => Err(nom_to_foil_error(&err, count_lines(buf))),
    }
}

fn nom_to_foil_error(err: &NomErr<&[u8]>, total_lines: usize) -> FoilErr {
    match err {
        &NomErr::Code(ref nom_err) => FoilErr::Code(to_foil_error_kind(nom_err)),
        &NomErr::Node(ref nom_err, ref err_vec) => {
            FoilErr::Node(to_foil_error_kind(nom_err),
                vec_nomerr_to_vec_foilerr(err_vec, total_lines))
        },
        &NomErr::Position(ref nom_err, ref text_left) => {
            FoilErr::Position(
                to_foil_error_kind(nom_err), 
                total_lines - count_lines(text_left))
        },
        &NomErr::NodePosition(ref nom_err, ref text_left, ref err_vec) => {
            FoilErr::NodePosition(
                to_foil_error_kind(nom_err),
                total_lines - count_lines(text_left),
                vec_nomerr_to_vec_foilerr(err_vec, total_lines))
        }
    }
}

fn vec_nomerr_to_vec_foilerr(err_vec: &Vec<NomErr<&[u8]>>, total_lines: usize) -> Vec<FoilErr> {
    let mut foilerr_vec = Vec::with_capacity(err_vec.len());
    for err in err_vec {
        foilerr_vec.push(nom_to_foil_error(err, total_lines));
    }
    foilerr_vec
}

pub fn count_lines(buf: &[u8]) -> usize {
    let mut lines = 0;
    let linebreak = '\n' as u8;
    for c in buf.iter() {
        if *c == linebreak {
            lines = lines + 1;
        }
    }

    lines
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_lines_works() {
        assert_eq!(3, count_lines(b"line1\nline2\nline3\nlast line"));
    }
}
