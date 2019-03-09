use crate::{
    error::Error,
    token::{Number, Token},
};
use std::str;

#[inline]
pub fn parse_number_radix(
    input: &[u8],
    c_src: &mut usize,
    token_len: u64,
    base: u8,
) -> Result<Token, Error> {
    let i =
        unsafe { str::from_utf8_unchecked(&input[*c_src - token_len as usize + 2..*c_src - 1]) };
    let i = u32::from_str_radix(i, u32::from(base))?;
    *c_src -= 1;
    Ok(Token::NumericLiteral(Number::new(i, 0, 1, base)))
}

#[inline]
pub fn parse_number(input: &[u8], c_src: &mut usize, token_len: u64) -> Result<Token, Error> {
    let i = unsafe { str::from_utf8_unchecked(&input[*c_src - token_len as usize..*c_src - 1]) };
    let i = u32::from_str_radix(i, 10)?;
    *c_src -= 1;
    Ok(Token::NumericLiteral(Number::new(i, 0, 1, 10)))
}

#[inline]
pub fn parse_number_decimal(
    input: &[u8],
    c_src: &mut usize,
    token_len: u64,
) -> Result<Token, Error> {
    let mut i_point = 0;
    for (i, item) in input
        .iter()
        .enumerate()
        .take(*c_src - 1)
        .skip(*c_src - token_len as usize)
    {
        if *item == b'.' {
            i_point = i;
            break;
        }
    }
    let integer = unsafe { str::from_utf8_unchecked(&input[*c_src - token_len as usize..i_point]) };
    let integer = u32::from_str_radix(integer, 10)?;

    let decimal = unsafe { str::from_utf8_unchecked(&input[i_point + 1..*c_src - 1]) };
    let decimal = u32::from_str_radix(decimal, 10)?;

    *c_src -= 1;
    Ok(Token::NumericLiteral(Number::new(integer, decimal, 1, 10)))
}

#[inline]
pub fn parse_exponent(input: &[u8], c_src: &mut usize, token_len: u64) -> Result<Token, Error> {
    let mut i_e = 0;
    let mut i_point = None;
    for (i, item) in input
        .iter()
        .enumerate()
        .take(*c_src - 1)
        .skip(*c_src - token_len as usize)
    {
        if *item == b'.' {
            i_point = Some(i);
        }
        if *item == b'e' || *item == b'E' {
            i_e = i;
            break;
        }
    }

    let (integer, decimal) = if i_point.is_some() {
        let integer = unsafe {
            str::from_utf8_unchecked(&input[*c_src - token_len as usize..i_point.unwrap()])
        };
        let integer = u32::from_str_radix(integer, 10)?;
        let decimal = unsafe { str::from_utf8_unchecked(&input[i_point.unwrap() + 1..i_e]) };
        (integer, u32::from_str_radix(decimal, 10)?)
    } else {
        let integer = unsafe { str::from_utf8_unchecked(&input[*c_src - token_len as usize..i_e]) };
        let integer = u32::from_str_radix(integer, 10)?;
        (integer, 0)
    };

    let exponent = unsafe { str::from_utf8_unchecked(&input[i_e + 1..*c_src - 1]) };
    let exponent = i64::from_str_radix(exponent, 10).unwrap();
    *c_src += 1;
    Ok(Token::NumericLiteral(Number::new(
        integer, decimal, exponent, 10,
    )))
}

#[cfg(test)]
mod tests {
    use super::super::*;
    should!(
        binary,
        "0b1 ",
        vec![Token::NumericLiteral(Number::new(1, 0, 1, 2)), Token::EOF]
    );

    should!(
        binary_capital,
        "0b1 ",
        vec![Token::NumericLiteral(Number::new(1, 0, 1, 2)), Token::EOF]
    );

    should!(
        binary_four,
        "0b110 ",
        vec![Token::NumericLiteral(Number::new(6, 0, 1, 2)), Token::EOF]
    );

    should!(
        octal,
        "0o7 ",
        vec![Token::NumericLiteral(Number::new(7, 0, 1, 8)), Token::EOF]
    );

    should!(
        octal_cpaital,
        "0O7 ",
        vec![Token::NumericLiteral(Number::new(7, 0, 1, 8)), Token::EOF]
    );

    should!(
        octal_eight,
        "0O110 ",
        vec![Token::NumericLiteral(Number::new(72, 0, 1, 8)), Token::EOF]
    );

    should!(
        hex,
        "0xa ",
        vec![Token::NumericLiteral(Number::new(10, 0, 1, 16)), Token::EOF]
    );

    should!(
        hex_capital,
        "0Xa ",
        vec![Token::NumericLiteral(Number::new(10, 0, 1, 16)), Token::EOF]
    );

    should!(
        hex_sixteen,
        "0x10 ",
        vec![Token::NumericLiteral(Number::new(16, 0, 1, 16)), Token::EOF]
    );

    should!(
        decimal,
        "01 ",
        vec![Token::NumericLiteral(Number::new(1, 0, 1, 10)), Token::EOF]
    );

    should!(
        decimal_ten,
        "10 ",
        vec![Token::NumericLiteral(Number::new(10, 0, 1, 10)), Token::EOF]
    );

    should!(
        decimaldigits,
        "10.1 ",
        vec![Token::NumericLiteral(Number::new(10, 1, 1, 10)), Token::EOF]
    );

    should!(
        decimaldigits_exponent_signed,
        "10.1e-2 ",
        vec![
            Token::NumericLiteral(Number::new(10, 1, -2, 10)),
            Token::EOF
        ]
    );

    should!(
        decimal_exponent_signed,
        "10e-2 ",
        vec![
            Token::NumericLiteral(Number::new(10, 0, -2, 10)),
            Token::EOF
        ]
    );

    should!(
        decimal_exponent_signed_plus,
        "10e+20 ",
        vec![
            Token::NumericLiteral(Number::new(10, 0, 20, 10)),
            Token::EOF
        ]
    );

    should!(
        decimal_exponent_unsigneds,
        "10e20 ",
        vec![
            Token::NumericLiteral(Number::new(10, 0, 20, 10)),
            Token::EOF
        ]
    );
}
