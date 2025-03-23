use std::fmt::Display;
use std::str::CharIndices;
use bitflags::bitflags;
use iced::futures::StreamExt;
use lazy_static::lazy_static;
use regex::bytes::Regex;

/// An enum to help the parser decide how to interpret a markdown document
pub enum MarkdownSpecification {
    /// The core markdown specification, with a boolean value to enable extended syntax
    Core(bool),

    /// The [CommonMark](https://commonmark.org/) specification
    CommonMark,

    /// The [GitHub](https://github.github.com/gfm/) specification
    GitHub,

    /// The [MarkdownExtra](https://michelf.ca/projects/php-markdown/extra/) specification
    MarkdownExtra,

    /// The [MultiMarkdown](https://fletcherpenney.net/multimarkdown/) specification
    MultiMarkdown,

    /// The [R Markdown](https://rmarkdown.rstudio.com/) specification
    RMarkdown,
}

bitflags! {
    pub struct MarkdownFeatureSet: u64 {
        const SPEC = 0b00001111_11111111;
    }
}

bitflags! {
    pub struct MarkdownFeatures: u64 {
        // CORE FEATURES
        const HEADINGS            = 0b00000000_00000000_00000000_00000001; //  1
        const PARAGRAPH           = 0b00000000_00000000_00000000_00000010; //  2
        const LINE_BREAKS         = 0b00000000_00000000_00000000_00000100; //  4
        const BOLD                = 0b00000000_00000000_00000000_00001000; //  8
        const ITALIC              = 0b00000000_00000000_00000000_00010000; // 16
        const BLOCK_QUOTES        = 0b00000000_00000000_00000000_00100000; // 32
        const LISTS               = 0b00000000_00000000_00000000_01000000; // 64
        const CODE                = 0b00000000_00000000_00000000_10000000; // 128
        const IMAGE               = 0b00000000_00000000_00000001_00000000; // 256
        const HORIZONTAL_RULES    = 0b00000000_00000000_00000010_00000000; // 512
        const LINKS               = 0b00000000_00000000_00000100_00000000; // 1024
        const HTML                = 0b00000000_00000000_00001000_00000000; // 2048 - currently unsupported

        // Extends Features
        const TABLES              = 0b00000000_00000000_00010000_00000000; // 4096
        const FENCED_CODE_BLOCKS  = 0b00000000_00000000_00100000_00000000; // 8192
        const FOOTNOTES           = 0b00000000_00000000_01000000_00000000; // 16384
        const HEADING_WITH_ID     = 0b00000000_00000000_10000000_00000000; // 32768
        const DEFINITION_LISTS    = 0b00000000_00000001_00000000_00000000; // 65536
        const STRIKETHROUGH       = 0b00000000_00000010_00000000_00000000;
        const TASK_LISTS          = 0b00000000_00000100_00000000_00000000;
        const EMOJI               = 0b00000000_00001000_00000000_00000000;
        const HIGHLIGHT           = 0b00000000_00010000_00000000_00000000;
        const SUBSCRIPT           = 0b00000000_00100000_00000000_00000000;
        const SUPERSCRIPT         = 0b00000000_01000000_00000000_00000000;
        const AUTO_LINK           = 0b00000000_10000000_00000000_00000000; // can be disabled using backticks

        // Markdown Extra syntax
        const ABBREVIATIONS       = 0b00000001_00000000_00000000_00000000; // see the docs: https://michelf.ca/projects/php-markdown/extra/#abbr

        // Common features people like:
        const LATEX               = 0b00000010_00000000_00000000_00000000; // not supported
        const IMAGE_CAPTIONS      = 0b00000100_00000000_00000000_00000000;
        const FRONTMATTER         = 0b00001000_00000000_00000000_00000000;
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TokenType {
    Whitespace,
    Heading(u8),
    Paragraph,
    LineBreak,
    Bold,
    Italic,
    BlockQuote,
    OrderedList,
    UnorderedList,
    CodeBlock,
    Image,
    HorizontalRule,
    Link,
    HTMLBlock,
    Table,
}


#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenType,
    pub value: String,
    pub vector: [usize; 2]
}

impl Token {
    fn whitespace(cursor: usize, count: usize) -> Self {
        Self::new(TokenType::Whitespace, String::from(' '), cursor, count)
    }

    fn heading(level: u8, title: String, cursor: usize, count: usize) -> Self {
        Self::new(TokenType::Heading(level), String::from(title), cursor, count)
    }

    fn new(kind: TokenType, value: String, cursor: usize, count: usize) -> Self {
        Token {
            kind,
            value,
            vector: [cursor, count]
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.kind {
            TokenType::Heading(level) => write!(f, "<h{}>{}</h{}>", level, self.value, level),
            _ => write!(f, "{:?}", self),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Tokenizer {
    pub source: String,
    pub cursor: usize,
}


#[derive(Debug)]
pub struct TokenizerError {
    pub message: String
}

lazy_static!(

    // Control characters (002, 003)
    static ref STX: char = char::from(0x2);
    static ref ETX: char = char::from(0x3);
);

pub type TokenizerResult = Result<Vec<Token>, TokenizerError>;

impl Tokenizer {
    pub fn new<S: Into<String>>(source: S) -> Tokenizer {
        Self {
            source: source.into(),
            cursor: 0,
        }
    }

    pub fn tokenize(&mut self) -> TokenizerResult {
        let mut tokens: Vec<Token> = vec![];
        let mut start_of_line: bool = true;
        let chars = self.source.chars().collect::<Vec<char>>();

        while self.cursor < self.source.len() {
            let (token, consumed) = match chars[self.cursor] {
                ' ' | '\t' => {
                    let count = self.peek_while(is_whitespace);
                    (Token::whitespace(self.cursor, count), count)
                },
                '\n' => {
                    let count = self.peek_while(|c| c == '\n') + 1;

                    start_of_line = true;

                    if count == 1 {
                        (Token::whitespace(self.cursor, count), 1)
                    } else {
                        (Token::new(TokenType::LineBreak, String::from("\n"), self.cursor, count), count)
                    }
                },
                // Match all title fields
                '#' => {
                    if !start_of_line {
                        self.cursor += 1;
                        tokens.push(
                            Token::new(
                                TokenType::Paragraph,
                                '#'.to_string(),
                                self.cursor,
                                1
                            )
                        );
                        continue;
                    }

                    start_of_line = false;


                    let hash_count = self.peek_while(|c| c == '#'); // account for first hash that triggered rule
                    let char_count  = self.peek_while(|c| c != '\n'); // account for final char in line and prior hashes

                    // The starting index is the index of the first character after the whitespace
                    let title_start = self.cursor + hash_count;
                    let title_end = self.cursor + char_count;

                    let mut src_str = self.source[title_start..title_end].to_string();
                    let is_valid = src_str.starts_with(' ');

                    if is_valid {
                        _ = src_str.remove(0); // remove leading whitespace
                        if hash_count >= 6 {
                            (Token::heading(6, src_str, self.cursor, char_count), char_count)
                        } else {
                            (Token::heading(hash_count as u8, src_str, self.cursor, char_count), char_count)
                        }
                    } else {
                        (
                            Token::new(
                                TokenType::Paragraph,
                                self.source[self.cursor..title_end].to_string(),
                                self.cursor,
                                char_count
                            ),
                            char_count
                        )
                    }
                },
                // '>' => {
                //     if !start_of_line {
                //         self.cursor += 1;
                //         tokens.push(
                //             Token::new(
                //                 TokenType::Paragraph,
                //                 '>'.to_string(),
                //                 self.cursor,
                //                 1
                //             )
                //         );
                //         continue;
                //     }
                // },
                // Match everything else
                x => {
                    (
                        Token::new(TokenType::Paragraph, char::from(x).to_string(), self.cursor, 1),
                        1
                    )
                }
            };

            self.cursor += consumed;
            tokens.push(token);
        }

        Ok(tokens)
    }

    // Greedily consume tokens based on a predicate, returning the total tokens consumed
    pub fn peek_while<P>(&mut self, predicate: P) -> usize
    where P: Fn(char) -> bool,
    {
        let mut count = 0;
        let chars = self.source.chars().collect::<Vec<char>>();

        while (self.cursor + count) < self.source.len() && predicate(chars[self.cursor + count]) {
            count += 1;
        }

        count
    }

    pub fn peek_while_surround<P>(&mut self, predicate: P) -> usize
    where P: Fn(char, char, char) -> bool,
    {
        let mut count = 0;
        let chars = self.source.chars().collect::<Vec<char>>();
        let etx_len = self.source.len()-1;
        loop {
            if !(self.cursor+count < self.source.len()) {
                break;
            }

            match self.cursor+count {
                0 => {
                    if self.source.len() >=2 {
                        if predicate(*STX, chars[self.cursor+count], chars[self.cursor+count+1]) {
                            count += 1;
                        } else {
                            break;
                        }
                    } else if self.source.len() == 0 {
                        break;
                    } else if self.source.len() == 1 {
                        if predicate(*STX, chars[self.cursor+count], *ETX) {
                            count += 1;
                        }
                    }
                }
                pos => {
                    if pos == etx_len {
                        if predicate(chars[pos-1], chars[pos], *ETX) {
                            count += 1;
                        }
                    } else {
                        if predicate(chars[pos-1], chars[pos], chars[pos+1]) {
                            count += 1;
                        }
                    }
                }
            }
        }


        count
    }
}


/// Generic predicate method for checking if a char is whitespace or not
fn is_whitespace(c: char) -> bool {
    c == ' ' || c == '\t'
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text() {
        let text = "Hello, World!";
        let mut tokenizer = Tokenizer::new(text);
        let outcome = tokenizer.tokenize();

        assert_ne!(outcome.is_err(), true);
    }

    // #[test]
    // fn test_whitespace() {
    //     let text = "Hello, World!";
    // }
    //
    // #[test]
    // fn test_paragraph() {
    //     let text = "Lorem ipsum dolor sit amet consectetur adipiscing elit";
    // }
    //
    mod headings {
        use super::super::*;
        #[test]
        fn level_1() {
            let text = "# Title\nsome text";
            let mut tokenizer = Tokenizer::new(text);
            let outcome = tokenizer.tokenize();
            assert_ne!(outcome.is_err(), true);
            let first_token = outcome.unwrap()[0].clone();
            assert_eq!(first_token.value, "Title");
            assert_eq!(first_token.vector, [0, 7]);
            assert_eq!(first_token.kind, TokenType::Heading(1));
            println!("{}", first_token)
        }


        #[test]
        fn level_2() {
            let text = "## Title\nsome text";
            let mut tokenizer = Tokenizer::new(text);
            let outcome = tokenizer.tokenize();
            assert_ne!(outcome.is_err(), true);
            let first_token = outcome.unwrap()[0].clone();
            assert_eq!(first_token.value, "Title");
            assert_eq!(first_token.vector, [0, 8]);
            assert_eq!(first_token.kind, TokenType::Heading(2));
            println!("{}", first_token)
        }


        #[test]
        fn level_3() {
            let text = "### Title\nsome text";
            let mut tokenizer = Tokenizer::new(text);
            let outcome = tokenizer.tokenize();
            assert_ne!(outcome.is_err(), true);
            let first_token = outcome.unwrap()[0].clone();
            assert_eq!(first_token.value, "Title");
            assert_eq!(first_token.vector, [0, 9]);
            assert_eq!(first_token.kind, TokenType::Heading(3));
            println!("{}", first_token)
        }


        #[test]
        fn level_4() {
            let text = "#### Title\nsome text";
            let mut tokenizer = Tokenizer::new(text);
            let outcome = tokenizer.tokenize();
            assert_ne!(outcome.is_err(), true);
            let first_token = outcome.unwrap()[0].clone();
            assert_eq!(first_token.value, "Title");
            assert_eq!(first_token.vector, [0, 10]);
            assert_eq!(first_token.kind, TokenType::Heading(4));
            println!("{}", first_token)
        }


        #[test]
        fn level_5() {
            let text = "##### Title\nsome text";
            let mut tokenizer = Tokenizer::new(text);
            let outcome = tokenizer.tokenize();
            assert_ne!(outcome.is_err(), true);
            let first_token = outcome.unwrap()[0].clone();
            assert_eq!(first_token.value, "Title");
            assert_eq!(first_token.vector, [0, 11]);
            assert_eq!(first_token.kind, TokenType::Heading(5));
            println!("{}", first_token)
        }

        #[test]
        fn level_6() {
            let text = "###### Title\nsome text";
            let mut tokenizer = Tokenizer::new(text);
            let outcome = tokenizer.tokenize();
            assert_ne!(outcome.is_err(), true);
            let first_token = outcome.unwrap()[0].clone();
            assert_eq!(first_token.value, "Title");
            assert_eq!(first_token.vector, [0, 12]);
            assert_eq!(first_token.kind, TokenType::Heading(6));
            println!("{}", first_token)
        }


        #[test]
        fn level_n() {
            let text = "########## Title\nsome text";
            let mut tokenizer = Tokenizer::new(text);
            let outcome = tokenizer.tokenize();
            assert_ne!(outcome.is_err(), true);
            let first_token = outcome.unwrap()[0].clone();
            assert_eq!(first_token.value, "Title" );
            assert_eq!(first_token.vector, [0, 16]);
            assert_eq!(first_token.kind, TokenType::Heading(6));
            println!("{}", first_token)
        }

        #[test]
        fn indented() {
            let text = "     # Title\nsome text";
            let mut tokenizer = Tokenizer::new(text);
            let outcome = tokenizer.tokenize();
            assert_ne!(outcome.is_err(), true);
            let heading_token = outcome.unwrap()[1].clone();
            assert_eq!(heading_token.value, "Title");
            assert_eq!(heading_token.vector, [5, 7]);
            assert_eq!(heading_token.kind, TokenType::Heading(1));
            println!("{}", heading_token)
        }

        #[test]
        fn nonconforming() {
            let text = "#example_title";
            let mut tokenizer = Tokenizer::new(text);
            let outcome = tokenizer.tokenize();
            assert_ne!(outcome.is_err(), true);
            let first_token = outcome.unwrap()[0].clone();
            assert_eq!(first_token.value, "#example_title");
            assert_eq!(first_token.vector, [0, 14]);
            assert_eq!(first_token.kind, TokenType::Paragraph);
            println!("{}", first_token)
        }
    }
}