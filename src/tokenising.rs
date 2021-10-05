use logos::{Logos, Lexer};
use smol_str::SmolStr;

#[repr(u16)]
#[derive(Logos, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Token {
    #[token("use")]
    Using,
    #[token("bind")]
    Binder,
    #[token("tighter")]
    Tighter,

    #[token("(")]
    LParen,
    #[token(")")]
    RParen,

    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,

    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,

    #[token("!")]
    Exclaimation,

    #[token("@")]
    PatternAt,

    #[token("#")]
    HashTag,

    #[token("$")]
    DollarSign,

    #[token("%")]
    Modulo,

    #[token("^")]
    UpperArrow,

    #[token("&")]
    Ampersand,

    #[token("*")]
    Asterisk,

    #[token("+")]
    Plus,
    
    #[token("-")]
    Minus,

    #[token("=")]
    EqualSign,

    #[token("_")]
    Underscore,

    #[regex("[<+->!@#%&=*]*" )]
    Symbols,

    #[regex("[_a-zA-Z][_a-zA-Z0-9]*")]
    Identifier,

    #[regex("[0-9][_0-9]*")]
    Number,

    #[regex("\"([^\"\\\\]|\\\\.)*\"")]
    #[regex("'([^'\\\\]|\\\\.)*'")]
    String,


    #[regex(" ")]
    Whitespace,
    #[regex("\t")]
    WhitespaceTab,
    #[token("\n")]
    Newline,

    // Group multiple whitespaces into one token
    #[regex(r"[ ]+")]
    Whitespaces,
    #[regex(r"[\t]+")]
    WhitespaceTabs,
    #[regex("[\n]+")]
    Newlines,

    #[error]
    Error
}

pub struct Tokeniser<'source> {
    lexer: Lexer<'source, Token>,
}

/// Tokeniser that distinguish which token needs to capture to save space.
impl<'source> Tokeniser<'source> {
    pub fn new(input: &'source str) -> Self {
        Self { lexer: Token::lexer(input) }
    }

    /// Source from which this Lexer is reading tokens, such as input string
    pub fn source(&self) -> &'source str {
        self.lexer.source()
    }

    fn need_capture(token: &Token) -> bool {
        match token {
            Token::Identifier => true,
            Token::Symbols => true,
            // Token::Integer => true,
            // Token::Float => true,
            Token::String => true,
            Token::Whitespaces => true,
            _ => false,
        }
    }
}

pub type TokenCluster = (Token, Option<SmolStr>);

impl Iterator for Tokeniser<'_> {
    type Item = TokenCluster;

    // Allocate the lexer text slice into stack allocate string only when it
    // needs to be captured.
    fn next(&mut self) -> Option<Self::Item> {
        let lex = self.lexer.next();
        match lex {
            None => None,
            Some(tok) => {
                if Tokeniser::need_capture(&tok) {
                    match tok {
                        // Truncated the trailing and heading \"\"
                        Token::String => {
                            let data: &str = self.lexer.slice().into();
                            let truncated_str = data.get(1..data.len()-1).unwrap_or_default();
                            Some((tok, Some(truncated_str.into())))
                        },
                        _ => Some((tok, Some(self.lexer.slice().into())))
                    }
                } else {
                    Some((tok, None))
                }
            }
        }
    }
}