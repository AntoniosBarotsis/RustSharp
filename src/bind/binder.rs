use crate::{ast::*, bind::bound_ast::Type};

use super::bound_ast::{
  binary_to_bound_binary_op, BoundExpr, BoundProgram, BoundStatement, ProgramError, TypeError,
};

#[derive(Default)]
pub struct Binder {
  bound_exprs: Vec<(String, BoundExpr)>,
}

impl Binder {
  /// Binds the program.
  pub fn bind(&mut self, prog: &Program) -> Result<BoundProgram, ProgramError> {
    match prog {
      Program::Body { stmts } => {
        let mut bound_statements = Vec::<BoundStatement>::new();
        let mut expr_errors = Vec::<TypeError>::new();

        for statement in stmts {
          match statement {
            Statement::Expr { expr } => match self.bind_expr(expr) {
              Ok(expr) => bound_statements.push(BoundStatement::BoundExpr { expr }),
              Err(err) => expr_errors.push(err),
            },
            Statement::Declare { identifier, rhs } => {
              let mut bound_names = self.bound_exprs.iter().map(|x| x.to_owned().0);
              if bound_names.any(|x| x == identifier.text) {
                let err = TypeError {
                  msg: "Variable identifier is already taken".to_owned(),
                  source_pos: SourcePos {
                    start: identifier.source_pos.start,
                    end: identifier.source_pos.end,
                  },
                };

                expr_errors.push(err);
                break;
              }

              match self.bind_expr(rhs) {
                Ok(rhs) => {
                  self
                    .bound_exprs
                    .push((identifier.to_owned().text, rhs.to_owned()));
                  let expr = BoundExpr::BoundDeclaration {
                    identifier: identifier.to_owned().text,
                    value_type: Self::get_type(&rhs),
                    rhs: Box::new(rhs),
                  };
                  bound_statements.push(BoundStatement::BoundExpr { expr })
                }
                Err(err) => expr_errors.push(err),
              }
            }
          }
        }

        if expr_errors.is_empty() {
          Ok(BoundProgram::Body {
            stmts: bound_statements,
          })
        } else {
          Err(ProgramError { expr_errors })
        }
      }
    }
  }

  /// Tries to bind an Expression.
  pub fn bind_expr(&mut self, expr: &Expr) -> Result<BoundExpr, TypeError> {
    match expr {
      Expr::Int { n, source_pos: _ } => Ok(BoundExpr::Int { n: n.to_owned() }),
      Expr::Bool { b, source_pos: _ } => Ok(BoundExpr::Bool { b: b.to_owned() }),
      Expr::ParenthesizedExpression { expr } => self.bind_expr(expr),
      Expr::BinaryOp { op, lhs, rhs } => {
        let source_pos = Self::get_src_pos_bin_op(op);
        let lhs_expr = self.bind_expr(lhs);
        let rhs_expr = self.bind_expr(rhs);

        match (lhs_expr, rhs_expr) {
          (Ok(lhs_expr), Ok(rhs_expr)) => {
            let lhs_type = Self::get_type(&lhs_expr);
            let rhs_type = Self::get_type(&rhs_expr);
            let (in_type, bin_op_type) = Self::get_op_type(op);

            let types_are_correct = lhs_type == rhs_type && lhs_type == in_type;

            match types_are_correct {
              true => {
                let lhs = Box::new(lhs_expr);
                let rhs = Box::new(rhs_expr);

                let tmp = BoundExpr::BoundBinaryOp {
                  op: binary_to_bound_binary_op(*op),
                  lhs,
                  rhs,
                  bin_op_type,
                };

                Ok(tmp)
              }
              false => Self::invalid_op_err(&lhs_type, &rhs_type, op, source_pos),
            }
          }
          (Ok(_), Err(err)) => Err(err),
          (Err(err), Ok(_)) => Err(err),
          (Err(err), Err(_)) => Err(err),
        }
      }
      Expr::Print { expr } => match *expr.to_owned() {
        Expr::Int { n, source_pos: _ } => Ok(BoundExpr::BoundPrint {
          expr_type: Type::Int,
          expr: Box::new(BoundExpr::Int { n }),
        }),
        Expr::Bool {
          b: _,
          source_pos: _,
        } => todo!(),
        Expr::String {
          text: str,
          source_pos: _,
        } => Ok(BoundExpr::BoundPrint {
          expr_type: Type::Int,
          expr: Box::new(BoundExpr::String { str }),
        }),
        Expr::BinaryOp { op, lhs, rhs } => {
          let source_pos = Self::get_src_pos(&Expr::BinaryOp { op, lhs, rhs });
          Err(TypeError {
            msg: "Can only print literals".to_owned(),
            source_pos,
          })
        }
        Expr::ParenthesizedExpression { expr } => {
          let source_pos = Self::get_src_pos(&expr);
          Err(TypeError {
            msg: "Can only print literals".to_owned(),
            source_pos,
          })
        }
        Expr::Print { expr } => {
          let source_pos = Self::get_src_pos(&expr);
          Err(TypeError {
            msg: "Can only print literals".to_owned(),
            source_pos,
          })
        }
        Expr::Identifier(identifier) => match self.retrieve_variable(&identifier) {
          Ok(expr) => {
            let value_type = Self::get_type(&expr);
            let expr = Box::new(expr);

            Ok(BoundExpr::BoundPrint {
              expr_type: value_type,
              expr,
            })
          }
          e @ Err(_) => e,
        },
      },
      Expr::String {
        text: str,
        source_pos: _,
      } => Ok(BoundExpr::String {
        str: str.to_owned(),
      }),
      Expr::Identifier(identifier) => self.retrieve_variable(identifier),
    }
  }

  /// Turns an `Identifier` into a `BoundExpr` or a `TypeError` if the variable
  /// is not defined.
  fn retrieve_variable(&self, identifier: &Identifier) -> Result<BoundExpr, TypeError> {
    match self.bound_exprs.iter().find(|x| x.0 == identifier.text) {
      Some(res) => {
        let identifier = identifier.text.to_owned();
        let value_type = Self::get_type(&res.1);
        let rhs = Box::new(res.1.to_owned());

        Ok(BoundExpr::BoundDeclaration {
          identifier,
          value_type,
          rhs,
        })
      }
      None => Err(TypeError {
        msg: "Variable is undefined".to_owned(),
        source_pos: identifier.source_pos,
      }),
    }
  }

  /// Returns the `Type` of the passed bound expression.
  fn get_type(expr: &BoundExpr) -> Type {
    match expr {
      BoundExpr::Int { n: _ } => Type::Int,
      BoundExpr::Bool { b: _ } => Type::Bool,
      BoundExpr::BoundBinaryOp {
        op: _,
        lhs: _,
        rhs: _,
        bin_op_type,
      } => bin_op_type.to_owned(),
      BoundExpr::ParenthesizedExpression { expr } => Self::get_type(expr),
      BoundExpr::BoundPrint { expr_type, expr: _ } => expr_type.to_owned(),
      BoundExpr::String { str: _ } => Type::String,
      BoundExpr::BoundDeclaration {
        identifier: _,
        value_type,
        rhs: _,
      } => value_type.to_owned(),
    }
  }

  /// Returns the source position of the passed expression.
  fn get_src_pos(expr: &Expr) -> SourcePos {
    match expr {
      Expr::Int { n: _, source_pos } => *source_pos,
      Expr::Bool { b: _, source_pos } => *source_pos,
      Expr::String {
        text: _,
        source_pos,
      } => *source_pos,
      Expr::Identifier(identifier) => identifier.source_pos,
      Expr::BinaryOp { op: _, lhs, rhs } => {
        let lhs_pos = Self::get_src_pos(lhs);
        let rhs_pos = Self::get_src_pos(rhs);

        SourcePos {
          start: lhs_pos.start,
          end: rhs_pos.end,
        }
      }
      Expr::ParenthesizedExpression { expr } => Self::get_src_pos(expr),
      Expr::Print { expr } => Self::get_src_pos(expr),
    }
  }

  /// `Self::get_src_pos` variant specific to bin ops.
  fn get_src_pos_bin_op(op: &'_ BinaryOp) -> &'_ SourcePos {
    match op {
      BinaryOp::Add { source_pos } => source_pos,
      BinaryOp::Sub { source_pos } => source_pos,
      BinaryOp::Mul { source_pos } => source_pos,
      BinaryOp::Div { source_pos } => source_pos,
      BinaryOp::And { source_pos } => source_pos,
      BinaryOp::Or { source_pos } => source_pos,
    }
  }

  /// Returns the operator's `(InputType, OutputType)`
  fn get_op_type(expr: &BinaryOp) -> (Type, Type) {
    match expr {
      BinaryOp::Add { source_pos: _ }
      | BinaryOp::Sub { source_pos: _ }
      | BinaryOp::Mul { source_pos: _ }
      | BinaryOp::Div { source_pos: _ } => (Type::Int, Type::Int),
      BinaryOp::And { source_pos: _ } | BinaryOp::Or { source_pos: _ } => (Type::Bool, Type::Bool),
    }
  }

  /// Helper method for throwing an invalid operator error.
  fn invalid_op_err(
    lhs: &Type,
    rhs: &Type,
    op: &BinaryOp,
    source_pos: &SourcePos,
  ) -> Result<BoundExpr, TypeError> {
    let op_type = op.to_string();
    let msg = format!(
      "Cannot perform '{}' between {:?} and {:?}.",
      op_type.to_lowercase(),
      lhs,
      rhs
    );
    let err = TypeError {
      msg,
      source_pos: source_pos.to_owned(),
    };
    Err(err)
  }
}
