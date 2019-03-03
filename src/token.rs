use internship::IStr;

/// Number representation of parsed number
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Number {
    integer: u32,
    decimal: u32,
    exponent: i128,
    base: u8,
}

impl Number {
    /// Create instance of js representaiton of number
    pub fn new(integer: u32, decimal: u32, exponent: i128, base: u8) -> Self {
        Self {
            integer,
            decimal,
            exponent,
            base,
        }
    }
}

/// Token that is results of consuming characters
#[derive(PartialEq, Debug, Clone)]
pub enum Token {
    /// &
    And,
    /// &=
    AndAssign,
    /// =
    Assign,
    /// =>
    AssignBigger,
    /// \>
    Bigger,
    /// \>=
    BiggerEqual,
    /// True / False
    BoolLiteral(bool),
    /// ^
    Caret,
    /// ^=
    CaretAssign,
    /// :
    Colon,
    /// ,
    Comma,
    /// ;
    Semicolon,
    /// /
    Slash,
    /// /=
    SlashAssign,
    /// .
    Dot,
    /// &&
    DoubleAnd,
    /// ==
    DoubleAssign,
    /// \\>>
    DoubleBigger,
    /// \\>>=
    DoubleBiggerEqual,
    /// <<
    DoubleLesser,
    /// <<=
    DoubleLesserEqual,
    /// --
    DoubleMinus,
    /// ||
    DoubleOr,
    /// ++
    DoublePlus,
    /// **
    DoubleStar,
    /// **=
    DoubleStarAssign,
    /// End of line
    EOF,
    /// /{this}/
    Regex(String),
    /// !
    Exclamation,
    /// !=
    ExclamationAssign,
    /// !==
    ExclamationDoubleAssign,
    /// a, name, (not keyword or reserved word)
    IdentifierName(IStr),
    /// async
    KAsync,
    /// await
    KAwait,
    /// break
    KBreak,
    /// case
    KCase,
    /// catch
    KCatch,
    /// class
    KClass,
    /// const
    KConst,
    /// continue
    KContinue,
    /// debugger
    KDebugger,
    /// default
    KDefault,
    /// delete
    KDelete,
    /// do
    KDo,
    /// else
    KElse,
    /// extend
    KExtend,
    /// finallly
    KFinally,
    /// for
    KFor,
    /// function
    KFunction,
    /// get
    KGet,
    /// if
    KIf,
    /// in
    KIn,
    /// let
    KLet,
    /// new
    KNew,
    /// of
    KOf,
    /// return
    KReturn,
    /// set
    KSet,
    /// static
    KStatic,
    /// switch
    KSwitch,
    /// this
    KThis,
    /// throw
    KThrow,
    /// try
    KTry,
    /// typeof
    KTypeof,
    /// var
    KVar,
    /// void
    KVoid,
    /// while
    KWhile,
    /// with
    KWith,
    /// {
    LCurly,
    /// <=
    LessEqual,
    /// <<
    Lesser,
    /// \n
    LineTerminator,
    /// null
    LNull,
    /// (
    LRound,
    /// [
    LSquare,
    /// \-
    Minus,
    /// \-=
    MinusAssign,
    /// 0 0.5 1e2 2.3e2
    NumericLiteral(Number),
    /// |
    Or,
    /// |=
    OrAssign,
    /// %
    Percent,
    /// %=
    PercentAssign,
    /// \+
    Plus,
    /// \+=
    PlusAssign,
    /// ?
    QuestionMark,
    /// }
    RCurly,
    /// )
    RRound,
    /// ]
    RSquare,
    /// \*
    Star,
    /// \*=
    StarAssign,
    /// "adad" 'adada'
    StringLiteral(String),
    /// \`ada{var}`
    Template(String),
    /// ~
    Tilde,
    /// ===
    TripleAssign,
    /// \>>>
    TripleBigger,
    /// \>>>=
    TripleBiggerEqual,
    /// ...
    TripleDot,
}
