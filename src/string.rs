use crate::token::Token;
use std::{char::from_u32, str};

// # Performance
// If string doesn't contain any escaping characaters we can skip
// allocation enitrely. This is acheived by allocating new string
// of size of input only after escaping character is encountered
#[inline]
fn to_unescaped(input: String) -> String {
    let mut s: Option<String> = None;
    let b = input.as_bytes();
    let mut escaping = false;
    let mut i = 0..b.len();
    loop {
        let next = i.next();
        if next == None {
            break;
        }
        let c = unsafe { b.get_unchecked(next.unwrap()) };
        if *c == b'\\' {
            escaping = true;
            if s.is_none() {
                s = Some(String::from(&input[..next.unwrap()]));
                s.as_mut().unwrap().reserve(input.len());
            }
            continue;
        }
        if escaping {
            escaping = false;
            let res = match *c as char {
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                'b' => '\x08',
                'v' => '\x0B',
                'f' => '\x0C',
                '0' => '\0',
                'u' => {
                    let index = i.next().unwrap();
                    let nums = &b[index..index + 4];
                    for _ in 0..3 {
                        i.next();
                    }
                    let as_num =
                        match u64::from_str_radix(unsafe { str::from_utf8_unchecked(nums) }, 16) {
                            Ok(v) => v,
                            Err(_) => 0,
                        };
                    match from_u32(as_num as u32) {
                        Some(v) => v,
                        None => panic!("{} is not a valid unicode scalar value", as_num),
                    }
                }
                'x' => {
                    let index = i.next().unwrap();
                    let nums = &b[index..index + 2];
                    for _ in 0..3 {
                        i.next();
                    }
                    let as_num =
                        match u64::from_str_radix(unsafe { str::from_utf8_unchecked(nums) }, 16) {
                            Ok(v) => v,
                            Err(_) => 0,
                        };
                    match from_u32(as_num as u32) {
                        Some(v) => v,
                        None => panic!("{} is not a valid unicode scalar value", as_num),
                    }
                }
                _ => *c as char,
            };
            s.as_mut().unwrap().push(res);
            continue;
        }
        if s.is_some() {
            s.as_mut().unwrap().push(*c as char);
        }
    }
    s.unwrap_or(input)
}

#[inline]
pub fn parse_string(input: &[u8], c_src: &mut usize, type_: u8) -> Token {
    let mut token_len = 0;
    while input.len() - 1 > *c_src && (input[*c_src] != type_ || input[*c_src - 1] == b'\\') {
        *c_src += 1;
        token_len += 1;
    }
    let res = unsafe { str::from_utf8_unchecked(&input[*c_src - token_len..*c_src]).to_string() };
    let res = to_unescaped(res);
    *c_src += 1;
    Token::StringLiteral(res)
}

#[inline]
pub fn parse_template(input: &[u8], c_src: &mut usize) -> Token {
    let mut token_len = 0;
    while input.len() - 1 > *c_src && (input[*c_src] != b'`' || input[*c_src - 1] == b'\\') {
        *c_src += 1;
        token_len += 1;
    }
    let res = unsafe { str::from_utf8_unchecked(&input[*c_src - token_len..*c_src]).to_string() };
    let res = to_unescaped(res);
    *c_src += 1;
    Token::Template(res)
}

#[cfg(test)]
mod tests {
    use super::super::*;

    should!(
        string_single,
        "'cau'",
        vec![Token::StringLiteral(String::from("cau")), Token::EOF]
    );

    should!(
        string_single_escape,
        "'c\\'au'",
        vec![Token::StringLiteral(String::from("c'au")), Token::EOF]
    );

    should!(
        string_single_unescape,
        "'\t'",
        vec![Token::StringLiteral(String::from("\t")), Token::EOF]
    );

    should!(
        string_double,
        r#""cau""#,
        vec![Token::StringLiteral(String::from("cau")), Token::EOF]
    );

    should!(
        string_double_escape,
        r#""c\"au""#,
        vec![Token::StringLiteral(String::from("c\"au")), Token::EOF]
    );

    should!(
        string_double_unescape,
        "\"\t\"",
        vec![Token::StringLiteral(String::from("\t")), Token::EOF]
    );

    should!(
        string_unicode,
        "\"\\u004E\"",
        vec![Token::StringLiteral(String::from("N")), Token::EOF]
    );

    should!(
        string_hex,
        "\"\\x4E\"",
        vec![Token::StringLiteral(String::from("N")), Token::EOF]
    );

    should!(
        template,
        "`cau`",
        vec![Token::Template(String::from("cau")), Token::EOF]
    );

    should!(
        template_escape,
        "`\\``",
        vec![Token::Template(String::from("`")), Token::EOF]
    );

    should!(
        template_unescape,
        "`\t`",
        vec![Token::Template(String::from("\t")), Token::EOF]
    );
}
