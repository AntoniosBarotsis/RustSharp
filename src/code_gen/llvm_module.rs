use crate::{
  bind::bound_ast::{BoundBinaryOp, BoundExpr, BoundProgram, BoundStatement, Type},
  code_gen::llvm_context::{GlobalVariable, LocalVariable, VarArgFunction},
};

use super::{
  llvm_context::{Instruction, LLVMStatement, LLVMType, Variable},
  llvm_ir_builder,
};

impl LLVMProgramBuilder {
  pub fn generate_llvm(&mut self, prog: BoundProgram) {
    let statements = match prog {
      BoundProgram::Body { stmts } => stmts,
    };

    let exprs: Vec<BoundExpr> = statements
      .iter()
      .map(|x| match x {
        BoundStatement::BoundExpr { expr } => expr.to_owned(),
      })
      .collect();

    if !self.main_instructions.is_empty() {
      // remove main
      self
        .main_instructions
        .remove(self.main_instructions.len() - 1);
    }

    for expr in exprs {
      let instruction = Self::bound_expr_to_llvm(expr.clone());
      match instruction {
        LLVMStatement::I32Literal(_) => todo!(),
        LLVMStatement::Addition(_, _) => todo!(),
        LLVMStatement::Print { expr_type, expr } => {
          if !self.print_information.include {
            self
              .global_instructions
              .push(Instruction::VarArgFunctionDeclaration(
                VarArgFunction::printf(),
              ));
            self.print_information.include = true;

            match expr_type {
              LLVMType::I8Ptr => todo!(),
              LLVMType::I32Ptr => todo!(),
              LLVMType::I8 => todo!(),
              LLVMType::I32 => match self.print_information.include_num {
                true => {}
                false => {
                  let format_num = GlobalVariable::format_num();

                  self
                    .global_instructions
                    .push(Instruction::GlobalVariableDeclaration(format_num));
                  self.print_information.include_num = true;
                }
              },
              LLVMType::Array(_) => todo!(),
            }
          }
          let str = llvm_ir_builder::stringify_llvm_statement(&expr);
          let str = match *expr {
            LLVMStatement::VariableDeclaration(_) => "%",
            _ => "",
          }
          .to_owned()
            + &str;

          self.main_instructions.push(Instruction::PrintNumber(str))
        }
        LLVMStatement::VariableDeclaration(variable) => {
          let name = variable.identifier;
          let value_type = variable.value_type;
          let value = llvm_ir_builder::stringify_llvm_statement(&variable.rhs);

          let var = LocalVariable {
            name,
            value_type,
            value,
          };
          self
            .main_instructions
            .push(Instruction::LocalVariableDeclaration(var))
        }
      }
    }

    self.main_instructions.push(Instruction::ReturnOk);

    println!("========");

    let tmp = self.build();

    match tmp {
      Ok(tmp) => println!("{}", tmp),
      Err(_) => todo!(),
    }

    println!("========");
  }

  fn bound_expr_to_llvm(expr: BoundExpr) -> LLVMStatement {
    match expr {
      BoundExpr::Int { n } => LLVMStatement::I32Literal(n),
      BoundExpr::BoundDeclaration {
        identifier,
        value_type,
        rhs,
      } => {
        let value_type = Self::type_to_llvm_type(value_type);
        let variable = Variable {
          identifier,
          value_type,
          rhs: Self::bound_expr_to_llvm(*rhs),
        };
        LLVMStatement::VariableDeclaration(Box::new(variable))
      }
      BoundExpr::Bool { b } => todo!(),
      BoundExpr::String { str } => todo!(),
      BoundExpr::BoundBinaryOp {
        op,
        lhs,
        rhs,
        bin_op_type: _,
      } => match op {
        BoundBinaryOp::Add => {
          let lhs = Box::new(Self::bound_expr_to_llvm(*lhs));
          let rhs = Box::new(Self::bound_expr_to_llvm(*rhs));
          LLVMStatement::Addition(lhs, rhs)
        }
        BoundBinaryOp::Sub => todo!(),
        BoundBinaryOp::Mul => todo!(),
        BoundBinaryOp::Div => todo!(),
        BoundBinaryOp::And => todo!(),
        BoundBinaryOp::Or => todo!(),
      },
      BoundExpr::ParenthesizedExpression { expr } => todo!(),
      BoundExpr::BoundPrint { expr_type, expr } => {
        let expr_type = Self::type_to_llvm_type(expr_type);
        let expr = Box::new(Self::bound_expr_to_llvm(*expr));

        LLVMStatement::Print { expr_type, expr }
      }
    }
  }

  fn build(&self) -> Result<String, String> {
    let mut res = String::new();

    match llvm_ir_builder::stringify_instructions(
      &self.global_instructions,
      &self.main_instructions,
    ) {
      Ok((global, main)) => {
        res.push_str(&global);
        res.push('\n');
        res.push_str(&main);

        Ok(res)
      }
      Err(e) => Err(e),
    }
  }

  fn type_to_llvm_type(value_type: Type) -> LLVMType {
    match value_type {
      Type::Int => LLVMType::I32,
      Type::Bool => todo!(),
      Type::String => todo!(),
    }
  }
}

#[derive(Default)]
pub struct LLVMProgramBuilder {
  print_information: PrintInformation,
  global_instructions: Vec<Instruction>,
  main_instructions: Vec<Instruction>,
}

#[derive(Default)]
struct PrintInformation {
  include: bool,
  include_num: bool,
}

#[derive(Debug)]
pub struct LLVMProgram {
  pub code: String,
}
