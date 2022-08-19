#![warn(clippy::unwrap_used)]

pub mod ast;
pub mod bind;
pub mod code_gen;

use bind::bound_ast::{BoundExpr, BoundProgram, BoundStatement, TypeError};
use parser::ProgramParser;

use crate::bind::bound_ast::BoundBinaryOp;

#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub parser);

const RED: &str = "\x1b[31m";
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const RBOLD: &str = "\x1b[22m";
const GRAY: &str = "\x1b[38;5;8m";

pub fn create_parser() -> ProgramParser {
  parser::ProgramParser::new()
}

fn print_expr(expr: BoundExpr, indent: &str, is_last: bool) {
  let marker = if is_last {
    "└───"
  } else {
    "├───"
  };

  print!("{}", indent);
  print!("{}", marker);

  match expr {
    BoundExpr::Int {
      n: _,
      // source_pos: _,
    } => print!("{}", green_text("Int")),
    BoundExpr::Bool {
      b: _,
      // source_pos: _,
    } => print!("{}", green_text("Bool")),
    BoundExpr::String { str: _ } => print!("{}", green_text("String")),
    BoundExpr::BoundBinaryOp {
      op,
      lhs: _,
      rhs: _,
      bin_op_type: _,
    } => {
      print!("{}", green_text("BinaryOp"));
      match op {
        BoundBinaryOp::Add => print!(" +"),
        BoundBinaryOp::Sub => print!(" -"),
        BoundBinaryOp::Mul => print!(" *"),
        BoundBinaryOp::Div => print!(" /"),
        BoundBinaryOp::And => print!(" &&"),
        BoundBinaryOp::Or => print!(" ||"),
      }
    }
    BoundExpr::ParenthesizedExpression { expr: _ } => {
      print!("{}", green_text("ParenthesizedExpression"))
    }
    BoundExpr::BoundPrint {
      expr_type: _,
      expr: _,
    } => print!("{}", green_text("Print:")),
    BoundExpr::BoundDeclaration {
      identifier: _,
      value_type: _,
      rhs: _,
    } => print!("{}", green_text("BoundDeclaration")),
  }

  let indent = indent.to_owned() + if is_last { "    " } else { "│   " };

  match expr {
    BoundExpr::Int { n } => println!("{}", red_text(&n.to_string())),
    BoundExpr::Bool { b } => println!("{}", red_text(&b.to_string())),
    BoundExpr::String { str } => println!("{}", red_text(&str)),
    BoundExpr::BoundBinaryOp {
      op: _,
      lhs,
      rhs,
      bin_op_type: _,
    } => {
      println!();
      print_expr(*lhs, &indent, false);
      print_expr(*rhs, &indent, true);
    }
    BoundExpr::ParenthesizedExpression { expr } => {
      println!();
      print_expr(*expr, &indent, true);
    }
    BoundExpr::BoundPrint { expr_type: _, expr } => {
      println!();
      print_expr(*expr, "", is_last);
    }
    BoundExpr::BoundDeclaration {
      identifier,
      value_type: _,
      rhs,
    } => {
      println!();
      print!("{}", &indent[0..]);
      println!("{}", red_text(&identifier));
      print_expr(*rhs, &indent, is_last)
    }
  }
}

/// Prints a tree like view of the passed expression.
pub fn print_program(prog: &BoundProgram) {
  match prog {
    BoundProgram::Body { stmts } => {
      for statement in stmts {
        match statement {
          BoundStatement::BoundExpr { expr } => print_expr(expr.to_owned(), "", true),
        }
      }
    }
  }
}

// https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797
fn red_text(str: &str) -> String {
  format!("\x1b[31m({})\x1b[0m", str)
}

fn green_text(str: &str) -> String {
  format!("\x1b[32m{}\x1b[0m", str.to_owned())
}

pub fn print_error(str: &str, err: TypeError) {
  // new err
  let start = err.source_pos.start;
  let end = err.source_pos.end;

  let lines = str.split('\n');
  let mut count = 0;
  let mut line_count = 1;

  for line in lines {
    if count + line.len() >= start {
      // Print error message
      println!(
        "\n{RED}ERROR{RESET}: {BOLD}{}{RBOLD} {GRAY}Ln {}{RESET}",
        err.msg, line_count
      );
      // Print erroneous line_count
      println!("{GRAY}{}{RESET}", line);

      // Print error indicator
      let before = " ".repeat(start - count);
      let err = "^".repeat((end - count) - (start - count));
      println!("{}{RED}{}{RESET}", before, err);
      break;
    } else {
      // Add line length + the \n we removed earlier
      count += line.len() + 1;
      line_count += 1;
    }
  }
}
