pub mod errors {
    #![allow(non_snake_case)]

    pub const ERROR_INVALID_TAG_NAME: u32 = 1;
    pub const ERROR_INVALID_ATTRIBUTE_NAME: u32 = 2;
    pub const ERROR_ATTRIBUTE_VALUE_NOT_STRING: u32 = 3;
    pub const ERROR_EXPECTED_WHITESPACE: u32 = 4;
    pub const ERROR_INVALID_EXPRESSION: u32 = 5;
    pub const ERROR_INVALID_DOM_NODE: u32 = 6;
    pub const ERROR_INVALID_SELF_CLOSING_DOM_NODE: u32 = 7;
    pub const ERROR_EXPECTING_DELIMITERS: u32 = 8;
    pub const ERROR_DOM_NODE: u32 = 9;
}
