use crate::token::Token;
use internship::IStr;
use std::str;

const IDENTIFIERS: phf::Map<&'static str, Token> = phf_map! {
    "true" => Token::BoolLiteral(true),
    "false" => Token::BoolLiteral(false),
    "null" => Token::LNull,
    "undefined" => Token::LUndefined,
    "function" => Token::KFunction,
    "async" => Token::KAsync,
    "class" => Token::KClass,
    "let" => Token::KLet,
    "if" => Token::KIf,
    "else" => Token::KElse,
    "do" => Token::KDo,
    "while" => Token::KWhile,
    "for" => Token::KFor,
    "var" => Token::KVar,
    "const" => Token::KConst,
    "in" => Token::KIn,
    "of" => Token::KOf,
    "await" => Token::KAwait,
    "switch" => Token::KSwitch,
    "case" => Token::KCase,
    "default" => Token::KDefault,
    "continue" => Token::KContinue,
    "break" => Token::KBreak,
    "return" => Token::KReturn,
    "with" => Token::KWith,
    "throw" => Token::KThrow,
    "try" => Token::KTry,
    "catch" => Token::KCatch,
    "finally" => Token::KFinally,
    "debugger" => Token::KDebugger,
    "extend" => Token::KExtend,
    "static" => Token::KStatic,
    "get" => Token::KGet,
    "set" => Token::KSet,
    "this" => Token::KThis,
    "delete" => Token::KDelete,
    "void" => Token::KVoid,
    "typeof" => Token::KTypeof,
    "new" => Token::KNew,
    "import" => Token::KImport,
    "as" => Token::KAs,
    "from" => Token::KFrom,
};

#[inline]
fn is_identifier_part(cp: u8) -> bool {
    cp == 0x24
        || cp == 0x5F
        || (cp >= 0x41 && cp <= 0x5A)
        || (cp >= 0x61 && cp <= 0x7A)
        || (cp >= 0x30 && cp <= 0x39)
        || cp == 0x5C
        || cp >= 0x80
}

#[inline]
pub fn parse_identifier(input: &[u8], c_src: &mut usize) -> Token {
    let mut it = 0;
    for i in 0..input.len() - *c_src {
        if !unsafe { is_identifier_part(*input.get_unchecked(*c_src + i)) } {
            it = i;
            break;
        }
    }
    let ident = &input[*c_src - 1..*c_src + it];
    *c_src += it;
    let ident = unsafe { str::from_utf8_unchecked(ident) };
    IDENTIFIERS
        .get(ident)
        .cloned()
        .unwrap_or_else(|| Token::IdentifierName(IStr::new(ident)))
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use ::internship::IStr;

    should!(
        parsing_after_token,
        "**=a ",
        vec![
            Token::DoubleStarAssign,
            Token::IdentifierName(IStr::new("a")),
            Token::EOF
        ]
    );

    should!(
        parsing_after_token_2,
        "a( a1 ",
        vec![
            Token::IdentifierName(IStr::new("a")),
            Token::LRound,
            Token::IdentifierName(IStr::new("a1")),
            Token::EOF
        ]
    );

    should!(keyword, "var ", vec![Token::KVar, Token::EOF]);
}
