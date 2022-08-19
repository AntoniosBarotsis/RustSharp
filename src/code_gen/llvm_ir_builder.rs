use super::llvm_context::{Instruction, LLVMStatement, LLVMType};

/// Returns a string tuple for `(Global, Main)` instructions.
/// This is very likely to change later as returning strings is weird.
/// Can also return string error message but that shouldn't happen.
pub(crate) fn stringify_instructions(
  global_instructions: Vec<Instruction>,
  main_instructions: Vec<Instruction>,
) -> Result<(String, String), String> {
  let mut global = String::new();
  let mut main = String::new();

  main.push_str("define i32 @main() {\n");

  for instruction in global_instructions {
    match instruction {
      Instruction::VarArgFunctionDeclaration(func) => {
        let str = format!(
          "declare {} @{}({}, ...)\n",
          func.out_type, func.name, func.in_types[0]
        );
        global.push_str(&str);
      }
      Instruction::GlobalVariableDeclaration(var) => {
        let str = format!(
          "@{} = {} {} {} {}\n",
          var.name,
          var.access,
          if var.is_constant { "constant" } else { "" },
          var.value_type,
          var.value
        );
        global.push_str(&str);
      }

      Instruction::LocalVariableDeclaration(_) => todo!(),
      Instruction::ReturnOk => todo!(),
      Instruction::PrintNumber(_) => todo!(),
    }
  }

  for instruction in main_instructions {
    match instruction {
      Instruction::PrintNumber(var) => {
        let str = format!(
          "call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([3 x i8], [3 x i8]* @format_num, i32 0, i32 0), i32 {})\n",
          var
        );
        main.push_str(&str);
      }
      Instruction::LocalVariableDeclaration(var) => {
        let str = match var.value_type {
          LLVMType::I8Ptr => todo!(),
          LLVMType::I32Ptr => todo!(),
          LLVMType::I8 => todo!(),
          LLVMType::I32 => {
            format!("%{} = {}\n", var.name, var.value)
          }
          LLVMType::Array(_) => todo!(),
        };
        main.push_str(&str);
      }
      Instruction::ReturnOk => main.push_str("ret i32 0"),
      Instruction::VarArgFunctionDeclaration(func) => todo!(),
      Instruction::GlobalVariableDeclaration(var) => todo!(),
    }
  }

  let mut main = main.replace('\n', "\n  ");
  main.push_str("\n}\n");

  Ok((global, main))
}

/// Stringifies a statement.
pub(crate) fn stringify_llvm_statement(expr: LLVMStatement) -> String {
  match expr {
    LLVMStatement::I32Literal(n) => format!("add i32 {}, 0", n),
    LLVMStatement::Print { expr_type, expr } => todo!(),
    LLVMStatement::Addition(x, y) => format!(
      "add i32 {}, {}",
      stringify_llvm_numeric(*x),
      stringify_llvm_numeric(*y)
    ),
    LLVMStatement::VariableDeclaration(v) => v.identifier,
  }
}

/// Stringifies a numeric statement.
pub(crate) fn stringify_llvm_numeric(expr: LLVMStatement) -> String {
  match expr {
    LLVMStatement::I32Literal(n) => n.to_string(),
    LLVMStatement::Addition(x, y) => todo!(),
    LLVMStatement::VariableDeclaration(v) => "%".to_owned() + &v.identifier,
    LLVMStatement::Print { expr_type, expr } => todo!(),
  }
}
