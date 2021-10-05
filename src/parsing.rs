//! Parser, to parse off-side rule and layout sensitive parsing

use bumpalo::{collections};

use crate::tokenising::TokenCluster;

#[cfg(feature = "serde")]
use serde::Serialize;

/// The source location, line and column number
pub struct LineCol(usize, usize);

/// The smallest unit of compilation process, aka a file
pub struct SourceNode<'a> {
    pub tokens: collections::Vec<'a, TokenCluster>,
    pub location: LineCol,
}

pub struct IdentifierNode /* <'a> */ {
    pub text: String,
    pub location: LineCol,
}

// #[cfg_attr(feature = "serde", derive(Serialize))]
// pub enum AstNode<'a> {
//     Identifier(IdentifierNode<'a>),
//     // Function(FunctionNode<'a>),
// }


// fn Parse(tokens: collections::Vec<TokenCluster>) -> SourceUnit {
//     for (token, val) in tokens.iter() {
//     }
//     SourceUnit { globalModuleInUse: vec![], body: vec![] }
// }