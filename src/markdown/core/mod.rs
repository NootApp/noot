#[derive(Debug, PartialEq)]
enum TokenType {
    Heading,
    Bold,
    Italic,
    BlockQuote,
    OrderedList,
    UnorderedList,
    Code,
    HorizontalRule,
    Link,
    Image,
}


#[derive(Debug, PartialEq)]
pub struct Token {
    token_type: TokenType,
    value: String,
}

impl Token {
    pub fn new(token_type: TokenType, value: String) -> Self {
        Token { token_type, value }
    }
}