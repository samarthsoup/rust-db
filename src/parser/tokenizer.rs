use std::iter::Peekable;
use std::str::Chars;

#[derive(PartialEq, Debug)]
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

#[derive(PartialEq, Debug)]
pub(crate) enum Whitespace {
    Space,
    Tab,
    Newline,
}

#[derive(PartialEq, Debug)]
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

#[derive(PartialEq, Debug)]
enum TokenizerError {
    KeywordOrIdentifierNotAscii,
    OperatorNotClosed(Token),
    UnexpectedAfterOperator{unexpected: char, operator: Token},
    QuoteSymbolNotClosed
}

fn tokenize(sql: &str) -> Result<Vec<Token>, TokenizerError>{
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut chars = sql.chars().peekable();
    let mut errata = Vec::new();
    
    while let Some(&ch) = chars.peek() {
        match ch {
            'A'..='Z' | 'a'..='z' => {
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
                
                let token_variant = keyword_or_identifier_str_to_enum_variant(&current_token)?;
                tokens.push(token_variant);
                current_token.clear();
            }
            '0'..='9' => {
                current_token.push(ch);
                chars.next();
                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_numeric() {
                        current_token.push(next_ch);
                        chars.next();
                    } else {
                        break;
                    }
                } 
                tokens.push(Token::Number(current_token.clone().into()));
                current_token.clear();
            }
            ' ' => tokenizer_consume_push(Token::Whitespace(Whitespace::Space), &mut chars, &mut tokens),
            '\t' => tokenizer_consume_push(Token::Whitespace(Whitespace::Tab), &mut chars, &mut tokens),
            '\n' => tokenizer_consume_push(Token::Whitespace(Whitespace::Newline), &mut chars, &mut tokens),
            '\r' => match chars.peek() {
                Some('\n') => tokenizer_consume_push(Token::Whitespace(Whitespace::Newline), &mut chars, &mut tokens),
                _ => tokens.push(Token::Whitespace(Whitespace::Newline))
            }
            '<' => match chars.peek() {
                Some('=') => tokenizer_consume_push(Token::LtEq, &mut chars, &mut tokens),
                _ => tokens.push(Token::Lt)
            }
            '>' => match chars.peek() {
                Some('=') => tokenizer_consume_push(Token::GtEq, &mut chars, &mut tokens),
                _ => tokens.push(Token::Gt)
            }
            '*' => tokenizer_consume_push(Token::Mul, &mut chars, &mut tokens),
            '/' => tokenizer_consume_push(Token::Div, &mut chars, &mut tokens),
            '+' => tokenizer_consume_push(Token::Plus, &mut chars, &mut tokens),
            '-' => tokenizer_consume_push(Token::Minus, &mut chars, &mut tokens),
            '=' => tokenizer_consume_push(Token::Eq, &mut chars, &mut tokens),
            '!' => match chars.peek() {
                Some('=') => tokenizer_consume_push(Token::Neq, &mut chars, &mut tokens),
                Some(unexpected) => {
                    let e = TokenizerError::UnexpectedAfterOperator{
                        unexpected: *unexpected,
                        operator: Token::Neq
                    };
                    errata.push(e);
                }
                None => errata.push(TokenizerError::OperatorNotClosed(Token::Neq))
            }
            '(' => tokenizer_consume_push(Token::LeftParen, &mut chars, &mut tokens),
            ')' => tokenizer_consume_push(Token::RightParen, &mut chars, &mut tokens),
            ',' => tokenizer_consume_push(Token::Comma, &mut chars, &mut tokens),
            ';' => tokenizer_consume_push(Token::SemiColon, &mut chars, &mut tokens),
            '"' | '\'' => tokenize_string(&mut chars, &mut tokens)?,
            _ => {

            }
        }
    }

    Ok(tokens)
}

fn tokenize_string(chars: &mut Peekable<Chars>, tokens: &mut Vec<Token>) -> Result<(), TokenizerError> {
    let quote_symbol = chars.next().unwrap();
    let mut string = String::new();
    while let Some(&ch) = chars.peek() {
        chars.next();
        if ch == quote_symbol {
            tokens.push(Token::String(string));
            return Ok(());
        }
        string.push(ch);
    }
    Err(TokenizerError::QuoteSymbolNotClosed)
}

fn tokenizer_consume_push(token_variant: Token, chars: &mut Peekable<Chars>, tokens: &mut Vec<Token> ) {
    chars.next();
    tokens.push(token_variant);
}

fn keyword_or_identifier_str_to_enum_variant(sql: &str) -> Result<Token, TokenizerError>{
    validate_string_ascii_alphanumeric_underscore(sql)?;

    let keyword = match sql.to_uppercase().as_str() {
        "SELECT" => Keyword::Select,
        "CREATE" => Keyword::Create,
        "UPDATE" => Keyword::Update,
        "DELETE" => Keyword::Delete,
        "INSERT" => Keyword::Insert,
        "VALUES" => Keyword::Values,
        "INTO" => Keyword::Into,
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
    };

    Ok(match keyword {
        Keyword::NaK => Token::Identifier(sql.to_string()),
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
    use crate::parser::tokenizer::{Keyword, Whitespace, Token};

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