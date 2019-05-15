extern crate javascript_lexer;
#[cfg(test)]
#[macro_use]
extern crate pretty_assertions;

use javascript_lexer::{
    internship,
    token::{Number, Token},
    Lexer,
};

#[test]
fn issue_1() {
    let parsed = Lexer::lex_tokens("import * as helpers from './helpers.js';").unwrap();
    use javascript_lexer::token::Token::*;
    assert_eq!(
        parsed,
        vec![
            KImport,
            Star,
            KAs,
            IdentifierName(internship::IStr::new("helpers")),
            KFrom,
            StringLiteral(String::from("./helpers.js")),
            Semicolon,
            EOF
        ]
    )
}
