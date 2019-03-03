use crate::{
    equivalence::{Equivalence, EQUIVALENCE_CLASS},
    error::Error,
    identifier,
    number::{parse_exponent, parse_number, parse_number_decimal, parse_number_radix},
    state::*,
    string,
    token::Token,
};
use std::str;

#[derive(Debug)]
struct StateMachine<S: State> {
    state: S,
}

impl<S: State> StateMachine<S> {
    fn new() -> StateMachine<InputElementDiv> {
        StateMachine {
            state: InputElementDiv,
        }
    }

    fn is_final(&self) -> bool {
        self.state.is_final()
    }
}

#[derive(Debug)]
enum StateMachineWrapper {
    LineTerminator(StateMachine<LineTerminator>),
    WhiteSpace(StateMachine<WhiteSpace>),
    SingleLineCommentAcc(StateMachine<SingleLineCommentAcc>),
    MultiLineCommentAcc(StateMachine<MultiLineCommentAcc>),
    DotPart(StateMachine<DotPart>),
    Comma(StateMachine<Comma>),
    CommaAcc(StateMachine<CommaAcc>),
    Semicolon(StateMachine<Semicolon>),
    SemicolonAcc(StateMachine<SemicolonAcc>),
    LesserAcc(StateMachine<LesserAcc>),
    BiggerAcc(StateMachine<BiggerAcc>),
    AssignAcc(StateMachine<AssignAcc>),
    ExclamationAcc(StateMachine<ExclamationAcc>),
    PlusAcc(StateMachine<PlusAcc>),
    MinusAcc(StateMachine<MinusAcc>),
    StarAcc(StateMachine<StarAcc>),
    PercentAcc(StateMachine<PercentAcc>),
    AndAcc(StateMachine<AndAcc>),
    OrAcc(StateMachine<OrAcc>),
    Tilde(StateMachine<Tilde>),
    TildeAcc(StateMachine<TildeAcc>),
    QuestionMark(StateMachine<QuestionMark>),
    QuestionMarkAcc(StateMachine<QuestionMarkAcc>),
    ColonAcc(StateMachine<ColonAcc>),
    CaretAcc(StateMachine<CaretAcc>),
    DoubleString(StateMachine<DoubleString>),
    SingleString(StateMachine<SingleString>),
    BinaryAcc(StateMachine<BinaryAcc>),
    OctalAcc(StateMachine<OctalAcc>),
    HexAcc(StateMachine<HexAcc>),
    DecimalAcc(StateMachine<DecimalAcc>),
    DecimalDigitsAcc(StateMachine<DecimalDigitsAcc>),
    DecimalExponentAcc(StateMachine<DecimalExponentAcc>),
    DecimalExponentSignedAcc(StateMachine<DecimalExponentSignedAcc>),
    Identifier(StateMachine<Identifier>),
    SlashAcc(StateMachine<SlashAcc>),
    LCurlyAcc(StateMachine<LCurlyAcc>),
    RCurlyAcc(StateMachine<RCurlyAcc>),
    LRoundAcc(StateMachine<LRoundAcc>),
    RRoundAcc(StateMachine<RRoundAcc>),
    LSquareAcc(StateMachine<LSquareAcc>),
    RSquareAcc(StateMachine<RSquareAcc>),
    Template(StateMachine<Template>),

    InputElementDiv(StateMachine<InputElementDiv>),
    Slash(StateMachine<Slash>),
    SingleLineComment(StateMachine<SingleLineComment>),
    MultiLineComment(StateMachine<MultiLineComment>),
    MultiLineCommentStar(StateMachine<MultiLineCommentStar>),
    Lesser(StateMachine<Lesser>),
    Bigger(StateMachine<Bigger>),
    Colon(StateMachine<Colon>),
    LCurly(StateMachine<LCurly>),
    LRound(StateMachine<LRound>),
    RRound(StateMachine<RRound>),
    LSquare(StateMachine<LSquare>),
    RSquare(StateMachine<RSquare>),
    RCurly(StateMachine<RCurly>),
    Assign(StateMachine<Assign>),
    Exclamation(StateMachine<Exclamation>),
    Plus(StateMachine<Plus>),
    Minus(StateMachine<Minus>),
    Star(StateMachine<Star>),
    Percent(StateMachine<Percent>),
    And(StateMachine<And>),
    Or(StateMachine<Or>),
    Caret(StateMachine<Caret>),
    SawZero(StateMachine<SawZero>),
    Binary(StateMachine<Binary>),
    Octal(StateMachine<Octal>),
    Hex(StateMachine<Hex>),
    Decimal(StateMachine<Decimal>),
    DecimalDigits(StateMachine<DecimalDigits>),
    DecimalExponent(StateMachine<DecimalExponent>),
    DecimalExponentSigned(StateMachine<DecimalExponentSigned>),
}

Edge!(InputElementDiv, LineTerminator);
Edge!(InputElementDiv, WhiteSpace);
Edge!(InputElementDiv, Slash);
Edge!(InputElementDiv, DotPart);
Edge!(Slash, SlashAcc);
Edge!(Slash, SingleLineComment);
Edge!(Slash, MultiLineComment);
Edge!(SingleLineComment, SingleLineCommentAcc);
Edge!(MultiLineComment, MultiLineCommentStar);
Edge!(MultiLineCommentStar, MultiLineCommentAcc);
Edge!(InputElementDiv, Identifier);
Edge!(InputElementDiv, LCurly);
Edge!(LCurly, LCurlyAcc);
Edge!(InputElementDiv, RCurly);
Edge!(RCurly, RCurlyAcc);
Edge!(InputElementDiv, LRound);
Edge!(LRound, LRoundAcc);
Edge!(InputElementDiv, RRound);
Edge!(RRound, RRoundAcc);
Edge!(InputElementDiv, LSquare);
Edge!(LSquare, LSquareAcc);
Edge!(InputElementDiv, RSquare);
Edge!(RSquare, RSquareAcc);
Edge!(InputElementDiv, Semicolon);
Edge!(Semicolon, SemicolonAcc);
Edge!(InputElementDiv, Comma);
Edge!(Comma, CommaAcc);
Edge!(InputElementDiv, Colon);
Edge!(Colon, ColonAcc);
Edge!(InputElementDiv, QuestionMark);
Edge!(QuestionMark, QuestionMarkAcc);
Edge!(InputElementDiv, Tilde);
Edge!(Tilde, TildeAcc);
Edge!(InputElementDiv, Lesser);
Edge!(Lesser, LesserAcc);
Edge!(InputElementDiv, Bigger);
Edge!(Bigger, BiggerAcc);
Edge!(InputElementDiv, Assign);
Edge!(Assign, AssignAcc);
Edge!(InputElementDiv, Exclamation);
Edge!(Exclamation, ExclamationAcc);
Edge!(InputElementDiv, Plus);
Edge!(Plus, PlusAcc);
Edge!(InputElementDiv, Star);
Edge!(Star, StarAcc);
Edge!(InputElementDiv, Percent);
Edge!(Percent, PercentAcc);
Edge!(InputElementDiv, Minus);
Edge!(Minus, MinusAcc);
Edge!(InputElementDiv, Or);
Edge!(Or, OrAcc);
Edge!(InputElementDiv, Caret);
Edge!(Caret, CaretAcc);
Edge!(InputElementDiv, SingleString);
Edge!(InputElementDiv, Template);
Edge!(InputElementDiv, DoubleString);
Edge!(InputElementDiv, And);
Edge!(And, AndAcc);
Edge!(InputElementDiv, SawZero);
Edge!(SawZero, Decimal);
Edge!(SawZero, DecimalAcc);
Edge!(InputElementDiv, Decimal);
Edge!(Decimal, DecimalAcc);
Edge!(Decimal, DecimalExponent);
Edge!(DecimalDigits, DecimalExponent);
Edge!(DecimalDigits, DecimalDigitsAcc);
Edge!(DecimalExponent, DecimalExponentSigned);
Edge!(DecimalExponentSigned, DecimalExponentSignedAcc);
Edge!(DecimalExponent, DecimalExponentAcc);
Edge!(Decimal, DecimalDigits);
Edge!(InputElementDiv, Hex);
Edge!(Hex, HexAcc);
Edge!(SawZero, Hex);
Edge!(InputElementDiv, Binary);
Edge!(Binary, BinaryAcc);
Edge!(SawZero, Binary);
Edge!(InputElementDiv, Octal);
Edge!(Octal, OctalAcc);
Edge!(SawZero, Octal);
Edge!(MultiLineCommentStar, MultiLineComment);
Edge!(LineTerminator, HELL);

impl StateMachineWrapper {
    #[inline]
    fn step(self, e: Equivalence) -> Self {
        include!("./transitions.rs")
    }

    #[inline]
    fn is_final(&self) -> bool {
        match self {
            StateMachineWrapper::LineTerminator(n) => n.is_final(),
            StateMachineWrapper::WhiteSpace(n) => n.is_final(),
            StateMachineWrapper::SingleLineCommentAcc(n) => n.is_final(),
            StateMachineWrapper::MultiLineCommentAcc(n) => n.is_final(),
            StateMachineWrapper::LCurly(n) => n.is_final(),
            StateMachineWrapper::LCurlyAcc(n) => n.is_final(),
            StateMachineWrapper::LRound(n) => n.is_final(),
            StateMachineWrapper::RRound(n) => n.is_final(),
            StateMachineWrapper::LSquare(n) => n.is_final(),
            StateMachineWrapper::RSquare(n) => n.is_final(),
            StateMachineWrapper::LRoundAcc(n) => n.is_final(),
            StateMachineWrapper::RRoundAcc(n) => n.is_final(),
            StateMachineWrapper::LSquareAcc(n) => n.is_final(),
            StateMachineWrapper::RSquareAcc(n) => n.is_final(),
            StateMachineWrapper::DotPart(n) => n.is_final(),
            StateMachineWrapper::Comma(n) => n.is_final(),
            StateMachineWrapper::CommaAcc(n) => n.is_final(),
            StateMachineWrapper::Semicolon(n) => n.is_final(),
            StateMachineWrapper::SemicolonAcc(n) => n.is_final(),
            StateMachineWrapper::LesserAcc(n) => n.is_final(),
            StateMachineWrapper::BiggerAcc(n) => n.is_final(),
            StateMachineWrapper::AssignAcc(n) => n.is_final(),
            StateMachineWrapper::ExclamationAcc(n) => n.is_final(),
            StateMachineWrapper::PlusAcc(n) => n.is_final(),
            StateMachineWrapper::MinusAcc(n) => n.is_final(),
            StateMachineWrapper::StarAcc(n) => n.is_final(),
            StateMachineWrapper::PercentAcc(n) => n.is_final(),
            StateMachineWrapper::AndAcc(n) => n.is_final(),
            StateMachineWrapper::OrAcc(n) => n.is_final(),
            StateMachineWrapper::Tilde(n) => n.is_final(),
            StateMachineWrapper::TildeAcc(n) => n.is_final(),
            StateMachineWrapper::QuestionMark(n) => n.is_final(),
            StateMachineWrapper::QuestionMarkAcc(n) => n.is_final(),
            StateMachineWrapper::ColonAcc(n) => n.is_final(),
            StateMachineWrapper::CaretAcc(n) => n.is_final(),
            StateMachineWrapper::DoubleString(n) => n.is_final(),
            StateMachineWrapper::SingleString(n) => n.is_final(),
            StateMachineWrapper::BinaryAcc(n) => n.is_final(),
            StateMachineWrapper::OctalAcc(n) => n.is_final(),
            StateMachineWrapper::HexAcc(n) => n.is_final(),
            StateMachineWrapper::DecimalAcc(n) => n.is_final(),
            StateMachineWrapper::DecimalDigitsAcc(n) => n.is_final(),
            StateMachineWrapper::DecimalExponentAcc(n) => n.is_final(),
            StateMachineWrapper::DecimalExponentSignedAcc(n) => n.is_final(),
            StateMachineWrapper::Identifier(n) => n.is_final(),
            StateMachineWrapper::SlashAcc(n) => n.is_final(),
            StateMachineWrapper::RCurly(n) => n.is_final(),
            StateMachineWrapper::RCurlyAcc(n) => n.is_final(),
            StateMachineWrapper::Template(n) => n.is_final(),

            StateMachineWrapper::InputElementDiv(n) => n.is_final(),
            StateMachineWrapper::Slash(n) => n.is_final(),
            StateMachineWrapper::SingleLineComment(n) => n.is_final(),
            StateMachineWrapper::MultiLineComment(n) => n.is_final(),
            StateMachineWrapper::Colon(n) => n.is_final(),
            StateMachineWrapper::MultiLineCommentStar(n) => n.is_final(),
            StateMachineWrapper::Lesser(n) => n.is_final(),
            StateMachineWrapper::Bigger(n) => n.is_final(),
            StateMachineWrapper::Assign(n) => n.is_final(),
            StateMachineWrapper::Exclamation(n) => n.is_final(),
            StateMachineWrapper::Plus(n) => n.is_final(),
            StateMachineWrapper::Minus(n) => n.is_final(),
            StateMachineWrapper::Star(n) => n.is_final(),
            StateMachineWrapper::Percent(n) => n.is_final(),
            StateMachineWrapper::And(n) => n.is_final(),
            StateMachineWrapper::Or(n) => n.is_final(),
            StateMachineWrapper::Caret(n) => n.is_final(),
            StateMachineWrapper::SawZero(n) => n.is_final(),
            StateMachineWrapper::Binary(n) => n.is_final(),
            StateMachineWrapper::Octal(n) => n.is_final(),
            StateMachineWrapper::Hex(n) => n.is_final(),
            StateMachineWrapper::Decimal(n) => n.is_final(),
            StateMachineWrapper::DecimalDigits(n) => n.is_final(),
            StateMachineWrapper::DecimalExponent(n) => n.is_final(),
            StateMachineWrapper::DecimalExponentSigned(n) => n.is_final(),
        }
    }
}

const TOKENS: phf::Map<&'static str, Token> = phf_map! {
    "{" => Token::LCurly,
    "}" => Token::RCurly,
    "(" => Token::LRound,
    ")" => Token::RRound,
    "," => Token::Comma,
    "[" => Token::LSquare,
    "]" => Token::RSquare,
    ":" => Token::Colon,
    "?" => Token::QuestionMark,
    "~" => Token::Tilde,
    "<" => Token::Lesser,
    "<<" => Token::DoubleLesser,
    "<=" => Token::LessEqual,
    "<<=" => Token::DoubleLesserEqual,
    ">" => Token::Bigger,
    ">=" => Token::BiggerEqual,
    ">>" => Token::DoubleBigger,
    ">>=" => Token::DoubleBiggerEqual,
    ">>>" => Token::TripleBigger,
    ">>>=" => Token::TripleBiggerEqual,
    "=" => Token::Assign,
    "=>" => Token::AssignBigger,
    "==" => Token::DoubleAssign,
    "===" => Token::TripleAssign,
    "!" => Token::Exclamation,
    "!=" => Token::ExclamationAssign,
    "!==" => Token::ExclamationDoubleAssign,
    "+" => Token::Plus,
    "+=" => Token::PlusAssign,
    "++" => Token::DoublePlus,
    "-" => Token::Minus,
    "-=" => Token::MinusAssign,
    "--" => Token::DoubleMinus,
    "*" => Token::Star,
    "*=" => Token::StarAssign,
    "**" => Token::DoubleStar,
    "**=" => Token::DoubleStarAssign,
    "%" => Token::Percent,
    "%=" => Token::PercentAssign,
    "&" => Token::And,
    "&=" => Token::AndAssign,
    "&&" => Token::DoubleAnd,
    "|" => Token::Or,
    "|=" => Token::OrAssign,
    "||" => Token::DoubleOr,
    "^" => Token::Caret,
    "^=" => Token::CaretAssign,
    ";" => Token::Semicolon,
};

#[inline]
pub fn parse(input: &str) -> Result<Vec<Token>, Error> {
    let mut st = StateMachineWrapper::InputElementDiv(StateMachine::<InputElementDiv>::new());
    let input = input.as_bytes();
    let mut tokens = Vec::with_capacity(input.len());

    let mut c_src: usize = 0;
    let mut token_len: u64 = 0;
    while c_src < input.len() {
        while !st.is_final() {
            let ch = unsafe { *input.get_unchecked(c_src) };
            st = st.step(EQUIVALENCE_CLASS[ch as usize]);
            c_src += 1;
            token_len += 1;
        }
        let token = &input[c_src - token_len as usize..c_src - 1];
        let token = unsafe { str::from_utf8_unchecked(token) };
        let token = TOKENS
            .get(token)
            .cloned()
            .or_else(|| state_match(st, input, &mut c_src, token_len).unwrap());
        c_src -= 1;
        if token.is_some() {
            tokens.push(token.unwrap());
        }

        st = StateMachineWrapper::InputElementDiv(StateMachine::<InputElementDiv>::new());
        token_len = 0;
    }
    Ok(tokens)
}

#[inline]
fn state_match(
    st: StateMachineWrapper,
    input: &[u8],
    c_src: &mut usize,
    token_len: u64,
) -> Result<Option<Token>, Error> {
    let res = match st {
        StateMachineWrapper::LineTerminator(_) => Some(Token::LineTerminator),
        // LF after comment is not considered to be part of comment
        // and should be left. We can parse it as part of singleline
        // comment and replace commen with line terminator
        StateMachineWrapper::SingleLineCommentAcc(_) => Some(Token::LineTerminator),
        StateMachineWrapper::MultiLineCommentAcc(_) => None,
        StateMachineWrapper::SlashAcc(_) => Some(parse_slash(input, c_src, token_len)),
        StateMachineWrapper::SingleString(_) => Some(string::parse_string(input, c_src, b'\'')),
        StateMachineWrapper::DoubleString(_) => Some(string::parse_string(input, c_src, b'"')),
        StateMachineWrapper::Template(_) => Some(string::parse_template(input, c_src)),
        StateMachineWrapper::BinaryAcc(_) => Some(parse_number_radix(input, c_src, token_len, 2)?),
        StateMachineWrapper::OctalAcc(_) => Some(parse_number_radix(input, c_src, token_len, 8)?),
        StateMachineWrapper::HexAcc(_) => Some(parse_number_radix(input, c_src, token_len, 16)?),
        StateMachineWrapper::DecimalAcc(_) => Some(parse_number(input, c_src, token_len)?),
        StateMachineWrapper::DotPart(_) => Some(parse_dot(input, c_src)),
        StateMachineWrapper::DecimalDigitsAcc(_) => {
            Some(parse_number_decimal(input, c_src, token_len)?)
        }
        StateMachineWrapper::DecimalExponentSignedAcc(_) => {
            Some(parse_exponent(input, c_src, token_len)?)
        }
        StateMachineWrapper::DecimalExponentAcc(_) => {
            Some(parse_exponent(input, c_src, token_len)?)
        }
        StateMachineWrapper::Identifier(_) => Some(identifier::parse_identifier(input, c_src)),
        _ => None,
    };
    *c_src += 1;
    Ok(res)
}

#[inline]
fn parse_dot(input: &[u8], c_src: &mut usize) -> Token {
    let rest_len = input.len() - *c_src;
    if rest_len >= 2 && input[*c_src] == b'.' && input[*c_src + 1] == b'.' {
        *c_src += 2;
        return Token::TripleDot;
    }

    Token::Dot
}

#[inline]
fn parse_slash(input: &[u8], c_src: &mut usize, token_len: u64) -> Token {
    let token_start = *c_src - token_len as usize;
    if token_len == 2 && input[token_start + 1] == b'=' {
        return Token::SlashAssign;
    }
    let mut it = 0;
    for i in 0..input.len() - *c_src {
        if !unsafe { *input.get_unchecked(*c_src + i) != b' ' } {
            it = i;
            break;
        }
    }
    let ident = &input[*c_src - 2..*c_src + it];
    *c_src += it;
    let ident = unsafe { str::from_utf8_unchecked(ident) };
    if ident.len() == 2 {
        return Token::Slash;
    }
    Token::Regex(String::from(ident))
}

#[cfg(test)]
mod tests {
    use super::super::*;

    should!(
        lineterminator_all,
        "\u{000A}\u{000D}",
        vec![Token::LineTerminator, Token::LineTerminator, Token::EOF]
    );

    should!(
        single_comment,
        "// rest // of comment \n",
        vec![Token::LineTerminator, Token::EOF]
    );

    should!(multi_comment, "/** rest // of comment */", vec![Token::EOF]);

    should!(
        multi_comment_with_newline,
        "/** rest // \n of comment */",
        vec![Token::EOF]
    );

    should!(left_curly, "{", vec![Token::LCurly, Token::EOF]);

    should!(right_curly, "}", vec![Token::RCurly, Token::EOF]);

    should!(left_round, "(", vec![Token::LRound, Token::EOF]);

    should!(right_round, ")", vec![Token::RRound, Token::EOF]);

    should!(left_square, "[", vec![Token::LSquare, Token::EOF]);

    should!(right_square, "]", vec![Token::RSquare, Token::EOF]);

    should!(dot, ".", vec![Token::Dot, Token::EOF]);

    should!(twodot, "..", vec![Token::Dot, Token::Dot, Token::EOF]);

    should!(tripledot, "...", vec![Token::TripleDot, Token::EOF]);

    should!(
        fourdot,
        "....",
        vec![Token::TripleDot, Token::Dot, Token::EOF]
    );

    should!(lesser, "< ", vec![Token::Lesser, Token::EOF]);

    should!(lesser_double, "<< ", vec![Token::DoubleLesser, Token::EOF]);

    should!(lesser_equal, "<= ", vec![Token::LessEqual, Token::EOF]);

    should!(
        lesser_equal_double,
        "<<= ",
        vec![Token::DoubleLesserEqual, Token::EOF]
    );

    should!(bigger, "> ", vec![Token::Bigger, Token::EOF]);

    should!(bigger_equal, ">= ", vec![Token::BiggerEqual, Token::EOF]);

    should!(bigger_double, ">> ", vec![Token::DoubleBigger, Token::EOF]);

    should!(bigger_triple, ">>> ", vec![Token::TripleBigger, Token::EOF]);

    should!(
        bigger_double_equal,
        ">>= ",
        vec![Token::DoubleBiggerEqual, Token::EOF]
    );

    should!(
        bigger_triple_equal,
        ">>>= ",
        vec![Token::TripleBiggerEqual, Token::EOF]
    );

    should!(assign, "= ", vec![Token::Assign, Token::EOF]);

    should!(assign_double, "== ", vec![Token::DoubleAssign, Token::EOF]);

    should!(assign_triple, "=== ", vec![Token::TripleAssign, Token::EOF]);

    should!(assign_bigger, "=> ", vec![Token::AssignBigger, Token::EOF]);

    should!(exclamation, "! ", vec![Token::Exclamation, Token::EOF]);

    should!(
        exclamation_assign,
        "!= ",
        vec![Token::ExclamationAssign, Token::EOF]
    );

    should!(
        exclamation_double_assing,
        "!== ",
        vec![Token::ExclamationDoubleAssign, Token::EOF]
    );

    should!(plus, "+ ", vec![Token::Plus, Token::EOF]);

    should!(plus_dobule, "++ ", vec![Token::DoublePlus, Token::EOF]);

    should!(plus_assign, "+= ", vec![Token::PlusAssign, Token::EOF]);

    should!(minus, "- ", vec![Token::Minus, Token::EOF]);

    should!(minus_double, "-- ", vec![Token::DoubleMinus, Token::EOF]);

    should!(minus_assign, "-= ", vec![Token::MinusAssign, Token::EOF]);

    should!(star, "* ", vec![Token::Star, Token::EOF]);

    should!(star_double, "** ", vec![Token::DoubleStar, Token::EOF]);

    should!(star_assign, "*= ", vec![Token::StarAssign, Token::EOF]);

    should!(
        star_double_assign,
        "**= ",
        vec![Token::DoubleStarAssign, Token::EOF]
    );

    should!(percent, "% ", vec![Token::Percent, Token::EOF]);

    should!(
        percent_assign,
        "%= ",
        vec![Token::PercentAssign, Token::EOF]
    );

    should!(and, "& ", vec![Token::And, Token::EOF]);

    should!(and_double, "&& ", vec![Token::DoubleAnd, Token::EOF]);

    should!(and_assign, "&= ", vec![Token::AndAssign, Token::EOF]);

    should!(or, "| ", vec![Token::Or, Token::EOF]);

    should!(or_double, "|| ", vec![Token::DoubleOr, Token::EOF]);

    should!(or_assign, "|= ", vec![Token::OrAssign, Token::EOF]);

    should!(tilde, "~ ", vec![Token::Tilde, Token::EOF]);

    should!(colon, ": ", vec![Token::Colon, Token::EOF]);

    should!(questionmark, "?", vec![Token::QuestionMark, Token::EOF]);

    should!(caret, "^ ", vec![Token::Caret, Token::EOF]);

    should!(caret_assign, "^= ", vec![Token::CaretAssign, Token::EOF]);

    should!(slash_assign, "/= ", vec![Token::SlashAssign, Token::EOF]);

    should!(slash, "/ ", vec![Token::Slash, Token::EOF]);

    should!(keyowrd, "var ", vec![Token::KVar, Token::EOF]);

    should!(
        regex,
        "/a/d ",
        vec![Token::Regex(String::from("/a/d")), Token::EOF]
    );

    should!(comma, ", ", vec![Token::Comma, Token::EOF]);

    // should_fail!(string_single, "'cau\n'",
    // vec![Token::StringLiteral(String::from("cau")), Token::EOF]);
}
