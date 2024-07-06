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
    SemiColon
}

enum TokenizerError {
    KeywordOrIdentifierNotAscii,
}

fn tokenize(sql: &str) {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut chars = sql.chars().peekable();
    
    while let Some(&ch) = chars.peek() {
        match ch {
            'A'..'Z' | 'a'..'z' => {
                current_token.push(ch);
                chars.next();
                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_alphanumeric() || next_ch == '_' {
                        current_token.push(next_ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                
                let token_variant = keyword_or_identifier_str_to_enum_variant(&current_token);
                tokens.push(token_variant);
                current_token.clear();
            }
        }
    }

}

fn keyword_or_identifier_str_to_enum_variant(sql: &str) -> Result<Token, TokenizerError>{
    validate_string_ascii_alphanumeric_underscore(sql)?;

    match value.to_uppercase() {
        "SELECT" => Keyword::Select,
        "CREATE" => Keyword::Create,
        "UPDATE" => Keyword::Update,
        "DELETE" => Keyword::Delete,
        "INSERT" => Keyword::Insert,
        "VALUES" => Keyword::Values,
        "INTO" => Keyword::Into,
        "SET" => Keyword::Set,
        "DROP" => Keyword::Drop,
        "FROM" => Keyword::From,
        "WHERE" => Keyword::Where,
        "PRIMARY" => Keyword::Primary,
        "KEY" => Keyword::Key,
        "UNIQUE" => Keyword::Unique,
        "TABLE" => Keyword::Table,
        "DATABASE" => Keyword::Database,
        "INT" => Keyword::Int,
        "BIGINT" => Keyword::BigInt,
        "UNSIGNED" => Keyword::Unsigned,
        "VARCHAR" => Keyword::Varchar,
        "BOOL" => Keyword::Bool,
        "TRUE" => Keyword::True,
        "FALSE" => Keyword::False,
        _ => Keyword::NaK
    }

    Ok(match keyword {
        Keyword::None => Token::Identifier(sql),
        _ => Token::Keyword(keyword),
    })
}

fn validate_string_ascii_alphanumeric_underscore(s: &str) -> Result<(), TokenizerError> {
    for c in s.chars() {
        if !c.is_ascii_alphanumeric() && c != '_' {
            return Err(TokenizerError::KeywordOrIdentifierNotAscii);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::tokenize;

    #[test]
    fn test_select() {
        let sql = "SELECT id, name FROM users;";
        let tokens = tokenize(sql);
        assert_eq!(
            tokens,
            Ok(vec![
                Token::Keyword(Keyword::Select),
                Token::Whitespace(Whitespace::Space),
                Token::Identifier("id".into()),
                Token::Comma,
                Token::Whitespace(Whitespace::Space),
                Token::Identifier("name".into()),
                Token::Whitespace(Whitespace::Space),
                Token::Keyword(Keyword::From),
                Token::Whitespace(Whitespace::Space),
                Token::Identifier("users".into()),
                Token::SemiColon,
            ])
        );
    }
}