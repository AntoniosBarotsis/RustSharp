use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Program {
  Body { stmts: Vec<Statement> },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Statement {
  Expr { expr: Expr },
  Declare { identifier: Identifier, rhs: Expr },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Identifier {
  pub text: String,
  pub source_pos: SourcePos,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
  Int {
    n: i32,
    source_pos: SourcePos,
  },
  Bool {
    b: bool,
    source_pos: SourcePos,
  },
  String {
    text: String,
    source_pos: SourcePos,
  },
  Identifier(Identifier),
  BinaryOp {
    op: BinaryOp,
    lhs: Box<Expr>,
    rhs: Box<Expr>,
  },
  ParenthesizedExpression {
    expr: Box<Expr>,
  },
  Print {
    expr: Box<Expr>,
  },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SourcePos {
  pub start: usize,
  pub end: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinaryOp {
  Add { source_pos: SourcePos },
  Sub { source_pos: SourcePos },
  Mul { source_pos: SourcePos },
  Div { source_pos: SourcePos },
  And { source_pos: SourcePos },
  Or { source_pos: SourcePos },
}

impl Display for BinaryOp {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      BinaryOp::Add { source_pos: _ } => write!(f, "add"),
      BinaryOp::Sub { source_pos: _ } => write!(f, "sub"),
      BinaryOp::Mul { source_pos: _ } => write!(f, "mul"),
      BinaryOp::Div { source_pos: _ } => write!(f, "div"),
      BinaryOp::And { source_pos: _ } => write!(f, "and"),
      BinaryOp::Or { source_pos: _ } => write!(f, "or"),
    }
  }
}
