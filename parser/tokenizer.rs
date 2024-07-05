enum Keyword {
    Select, 
    Create,
    Update,
    Delete,
    Insert,
    Into,
    Values,
    Drop,
    From,
    Where,
    Primary,
    Key,
    Unique,
    Table,
    Database,
    Int,
    BigInt,
    Unsigned,
    Varchar,
    Bool,
    True,
    False,
    NaK //Not a Keyword (used for _ case in pattern matching)
}

pub(crate) enum Whitespace {
    Space,
    Tab,
    Newline,
}

enum Token {
    Keyword(Keyword),
    Identifier(String),
    Whitespace(Whitespace),
    String(String),
    Number(String),
    Eq,
    Neq,
    Lt,
    Gt,
    LtEq,
    GtEq,
    Mul,
    Div,
    Plus,
    Minus,
    LeftParen,
    RightParen,
    Comma,
    SemiColon,
    EoF, //to show end of the stream
}