use crate::ast::{BinaryOp, SourcePos};

#[derive(Debug)]
pub struct ProgramError {
  pub expr_errors: Vec<TypeError>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeError {
  pub msg: String,
  pub source_pos: SourcePos,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BoundProgram {
  Body { stmts: Vec<BoundStatement> },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BoundStatement {
  BoundExpr { expr: BoundExpr },
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Type {
  Int,
  Bool,
  String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum BoundExpr {
  BoundDeclaration {
    identifier: String,
    value_type: Type,
    rhs: Box<BoundExpr>,
  },
  Int {
    n: i32,
  },
  Bool {
    b: bool,
  },
  String {
    str: String,
  },
  BoundBinaryOp {
    op: BoundBinaryOp,
    lhs: Box<BoundExpr>,
    rhs: Box<BoundExpr>,
    bin_op_type: Type,
  },
  ParenthesizedExpression {
    expr: Box<BoundExpr>,
  },
  BoundPrint {
    expr_type: Type,
    expr: Box<BoundExpr>,
  },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoundBinaryOp {
  Add,
  Sub,
  Mul,
  Div,
  And,
  Or,
}

pub fn binary_to_bound_binary_op(op: BinaryOp) -> BoundBinaryOp {
  match op {
    BinaryOp::Add { source_pos: _ } => BoundBinaryOp::Add,
    BinaryOp::Sub { source_pos: _ } => BoundBinaryOp::Sub,
    BinaryOp::Mul { source_pos: _ } => BoundBinaryOp::Mul,
    BinaryOp::Div { source_pos: _ } => BoundBinaryOp::Div,
    BinaryOp::And { source_pos: _ } => BoundBinaryOp::Add,
    BinaryOp::Or { source_pos: _ } => BoundBinaryOp::Or,
  }
}
