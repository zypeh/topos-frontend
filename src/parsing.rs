//! Parser, to parse off-side rule and layout sensitive parsing

use bumpalo::{Bump, collections};
use smol_str::SmolStr;

use crate::tokenising::{TokenCluster, Token};

#[cfg(feature = "serde")]
use serde::Serialize;

/// The source location, line and column number
#[derive(Debug)]
pub struct LineCol(usize, usize);

impl LineCol {
    pub fn add_offset(&mut self, x: usize) -> Self {
        self.1 += x;
        LineCol(self.0, self.1)
    }

    pub fn surrounding_offset(&mut self, len: usize) -> Self {
        self.1 = self.1 + len + 2;
        LineCol(self.0, self.1)
    }
}

/// The smallest unit of compilation process, aka a file
pub struct SourceNode<'a> {
    pub tokens: collections::Vec<'a, TokenCluster>,
    /// Main
    pub location: &'a mut LineCol,
}

#[derive(Debug)]
pub struct IdentifierNode {
    pub text: SmolStr,
    pub location: LineCol,
}

#[derive(Debug)]
pub struct StringNode {
    pub text: SmolStr,
    pub location: LineCol,
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug)]
pub enum AstNode {
    Identifier(IdentifierNode),
    String(StringNode),
    // Function(FunctionNode<'a>),
}

pub struct Parser;

impl<'a> Parser {
    pub fn parse(tokens: collections::Vec<'a, TokenCluster>) -> Vec<AstNode> {
        let mut line_col = LineCol(1, 0);

        let mut ast_stream = vec![];
        for token in tokens.into_iter() {
            match token {
                // Check need_capture() function
                (Token::Identifier, Some(val)) => {
                    let ast = AstNode::Identifier(IdentifierNode{ text: val.clone(), location: line_col.add_offset(val.len()) });
                    ast_stream.push(ast)
                },
                (Token::Identifier, None) => unreachable!("uncaptured identifier"),
                (Token::Symbols, Some(val)) => {
                    let ast = AstNode::Identifier(IdentifierNode{ text: val.clone(), location: line_col.add_offset(val.len()) });
                    ast_stream.push(ast)
                },
                (Token::Symbols, None) => unreachable!("uncaptured symbols"),
                (Token::String, Some(val)) => {
                    let ast = AstNode::String(StringNode{ text: val.clone(), location: line_col.surrounding_offset(val.len()) });
                    ast_stream.push(ast)
                },
                (Token::String, None) => unreachable!("uncaptured string"),
                (Token::Whitespace, None) => { line_col.add_offset(1); },
                (Token::Whitespace, Some(_)) => unreachable!("whitespace does not have any value captured"),
                _ => unreachable!("a")
            };
        }
        ast_stream
    }  
}
