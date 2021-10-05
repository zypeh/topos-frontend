//! Parser, to parse off-side rule and layout sensitive parsing

use bumpalo::{collections};
use smol_str::SmolStr;

use crate::tokenising::{Token, TokenCluster};

/// The source location, line and column number
#[derive(Debug, Clone, Copy)]
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

    pub fn newline_offset(&mut self) -> Self {
        self.0 += 1;
        self.1 = 0;
        LineCol(self.0, self.1)
    }

    pub fn multiple_newline_offset(&mut self, n: usize) -> Self {
        self.0 += n;
        self.1 = 0;
        LineCol(self.0, self.1)
    }
}

/// The smallest unit of compilation process, aka a file
pub struct SourceNode<'a> {
    pub tokens: collections::Vec<'a, TokenCluster>,
    /// Main
    pub location: &'a mut LineCol,
}

#[derive(Debug, Clone)]
pub struct IdentifierNode {
    pub text: SmolStr,
    pub location: LineCol,
}

#[derive(Debug, Clone)]
pub struct StringNode {
    pub text: SmolStr,
    pub location: LineCol,
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[derive(Debug, Clone)]
pub enum ValueNode {
    Identifier(IdentifierNode),
    String(StringNode),
    /// This is for the incomplete tree.
    Hole,
}

#[derive(Debug, Clone)]
pub struct AssignmentNode {
    pub assignee: ValueNode,
    pub expression: ExpressionNode,
}

#[derive(Debug, Clone)]
pub enum StatementNode {
    Assignment(AssignmentNode),
}

#[derive(Debug, Clone)]
pub struct BinaryOpsNode {
    operand: IdentifierNode,
    left: ValueNode,
    right: ValueNode,
}

#[derive(Debug, Clone)]
pub enum ExpressionNode {
    Value(ValueNode),
    BinaryOps(BinaryOpsNode),
}

#[derive(Debug, Clone)]
pub enum AstRoot {
    Expression(ExpressionNode),
    Statement(StatementNode),
}

impl Default for AstRoot {
    fn default() -> Self {
        AstRoot::Expression(ExpressionNode::Value(ValueNode::Hole))
    }
}

#[derive(Debug, Clone)]
pub struct Parser {
    token_iterator: std::iter::Peekable<std::vec::IntoIter<TokenCluster>>,
    iteration: usize,
    line_col: LineCol,
    pub state: AstRoot,
}

impl<'a> Parser{
    pub fn new(tokens: collections::Vec<'a, TokenCluster>) -> Self {
        let token_iterator = tokens.to_vec().into_iter().peekable();
        Parser {
            token_iterator,
            iteration: 0,
            line_col: LineCol(1, 0),
            state: Default::default()
        }
    }

    pub fn consume(&mut self) -> &mut Self {
        if let Some(token) = self.token_iterator.next() {
            match token {
                (Token::Identifier, Some(val)) | (Token::Symbols, Some(val)) => {
                    self.state = AstRoot::Expression(ExpressionNode::Value(ValueNode::Identifier(
                        IdentifierNode {
                            text: val.clone(),
                            location: self.line_col.add_offset(val.len())
                        })));
                    self
                },
                (Token::String, Some(val)) => {
                    self.state = AstRoot::Expression(ExpressionNode::Value(ValueNode::String(
                        StringNode {
                            text: val.clone(),
                            location: self.line_col.surrounding_offset(val.len())
                        })));
                    self
                },
                (Token::Whitespace, None) => { self.line_col.add_offset(1); self },
                (Token::Newline, None) => { self.line_col.newline_offset(); self },
                _ => unimplemented!("sorry unimplemented")
            }
        } else {
            self
        }
    }

    /// For initial execution, incrementally_build is equivalent to consume.
    /// This function will combine the state maintained in the parser and combine with the
    /// token parsed. This is similar to the Rowan (Red-Green Tree)
    pub fn incrementally_build(&'a mut self) -> &mut Self {
        if let Some(token) = self.token_iterator.peek() {
            match (self.state.clone(), token) {
                (AstRoot::Expression(ExpressionNode::Value(ValueNode::Hole)), _) => self.consume(),
                (AstRoot::Expression(ExpressionNode::Value(ValueNode::Identifier(ident))), (Token::EqualSign, _)) => {
                    println!("what is next? {:?}", self.token_iterator.next());
                    
                    match self.consume().state.clone() {
                        AstRoot::Expression(ExpressionNode::Value(val)) => {
                            self.state = AstRoot::Statement(StatementNode::Assignment(
                                AssignmentNode {
                                    assignee: ValueNode::Identifier(ident),
                                    expression: ExpressionNode::Value(val),
                                }
                            ));
                            return self
                        },
                        _ => unimplemented!("la mer, unfinished.")
                    }
                }
                (x, y) => {println!("what is x: {:?}, and what is y: {:?}", x, y); self}
            }
        } else {
            self
        }
    }
}

