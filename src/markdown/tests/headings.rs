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