use crate::markdown::TokenType::Text;
use super::super::*;


// BOLD
#[test]
fn bold_asterisk() {
    let sample = "**this is bold**, this is not";
    let mut tokenizer = Tokenizer::new(sample);
    let outcome = tokenizer.tokenize().unwrap();
    let first_token = outcome.first().unwrap();
    match first_token.kind {
        TokenType::Text(modifier) => assert_eq!(modifier, TextModifier::BOLD),
        _ => assert!(false),
    }
    assert_eq!(first_token.value, "this is bold");
}

// This test is disabled as we will be supporting strikethrough
// and underlined text using similar syntax

// #[test]
// fn bold_underscore() {
//     let sample = "__this is bold__, this is not";
//     let mut tokenizer = Tokenizer::new(sample);
//     let outcome = tokenizer.tokenize().unwrap();
//     let first_token = outcome.first().unwrap();
//     match first_token.kind {
//         TokenType::Text(modifier) => assert_eq!(modifier, TextModifier::BOLD),
//         _ => assert!(false),
//     }
//     assert_eq!(first_token.value, "this is bold");
// }

// ITALICS

#[test]
fn italic_asterisk() {
    let sample = "*this is italic*, this is not";
    let mut tokenizer = Tokenizer::new(sample);
    let outcome = tokenizer.tokenize().unwrap();
    let first_token = outcome.first().unwrap();
    match first_token.kind {
        TokenType::Text(modifier) => assert_eq!(modifier, TextModifier::ITALIC),
        _ => assert!(false),
    }
}

#[test]
fn italic_underscore() {
    let sample = "_this is italic_, this is not";
    let mut tokenizer = Tokenizer::new(sample);
    let outcome = tokenizer.tokenize().unwrap();
    let first_token = outcome.first().unwrap();
    match first_token.kind {
        TokenType::Text(modifier) => assert_eq!(modifier, TextModifier::ITALIC),
        _ => assert!(false),
    }
}

// The following test is disabled because we will be supporting
// subscript text using the single tilde syntax

// #[test]
// fn italic_tilde() {
//     let sample = "~this is italic~, this is not";
//     let mut tokenizer = Tokenizer::new(sample);
//     let outcome = tokenizer.tokenize().unwrap();
//     let first_token = outcome.first().unwrap();
//     match first_token.kind {
//         TokenType::Text(modifier) => assert_eq!(modifier, TextModifier::ITALIC),
//         _ => assert!(false),
//     }
// }
//
// STRIKETHROUGH
#[test]
fn strikethrough_tilde() {
    let sample = "~~this is strikethrough~~, this is not";
    let mut tokenizer = Tokenizer::new(sample);
    let outcome = tokenizer.tokenize().unwrap();
    let first_token = outcome.first().unwrap();
    match first_token.kind {
        TokenType::Text(modifier) => assert_eq!(modifier, TextModifier::STRIKETHROUGH),
        _ => assert!(false),
    }
}


// COMBINATIONS
#[test]
fn bold_and_italic() {
    let expected = (TextModifier::BOLD | TextModifier::ITALIC);
    println!("Expecting modifier: {:?}", expected);
    let sample = "***this is bold and italic***, this is not";
    let mut tokenizer = Tokenizer::new(sample);
    let outcome = tokenizer.tokenize().unwrap();
    let first_token = outcome.first().unwrap();
    match first_token.kind {
        TokenType::Text(modifier) => assert_eq!(modifier, expected),
        _ => assert!(false),
    }
}