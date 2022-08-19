// contains the internal LLVM data structures

use std::fmt;

#[derive(Clone, Debug)]
pub enum Instruction {
  VarArgFunctionDeclaration(VarArgFunction),
  PrintNumber(String),
  GlobalVariableDeclaration(GlobalVariable),
  LocalVariableDeclaration(LocalVariable),
  // LoadLocalVariable {
  //   name: String,
  //   variable: LocalVariable,
  // },
  ReturnOk,
}

#[derive(Clone, Debug)]
pub enum LLVMStatement {
  I32Literal(i32),
  Addition(Box<LLVMStatement>, Box<LLVMStatement>),
  VariableDeclaration(Box<Variable>),
  Print {
    expr_type: LLVMType,
    expr: Box<LLVMStatement>,
  },
}

#[derive(Clone, Debug)]
pub struct Variable {
  pub identifier: String,
  pub value_type: LLVMType,
  pub rhs: LLVMStatement,
}

#[derive(Clone, Debug)]
pub struct VarArgFunction {
  pub out_type: LLVMType,
  pub name: String,
  pub in_types: Vec<LLVMType>,
}

impl VarArgFunction {
  pub fn new(out_type: LLVMType, name: String, in_types: Vec<LLVMType>) -> Self {
    Self {
      out_type,
      name,
      in_types,
    }
  }

  pub fn printf() -> Self {
    Self {
      out_type: LLVMType::I32,
      name: "printf".to_owned(),
      in_types: vec![LLVMType::I8Ptr],
    }
  }
}

#[derive(Clone, Debug)]
pub struct GlobalVariable {
  pub name: String,
  pub access: AccessModifier,
  pub is_constant: bool,
  pub value_type: LLVMType,
  pub value: String,
}

impl GlobalVariable {
  pub fn new(
    name: String,
    access: AccessModifier,
    is_constant: bool,
    value_type: LLVMType,
    value: String,
  ) -> Self {
    Self {
      name,
      access,
      is_constant,
      value_type,
      value,
    }
  }

  pub fn format_num() -> Self {
    Self {
      name: "format_num".to_owned(),
      access: AccessModifier::Private,
      is_constant: true,
      value_type: LLVMType::Array(Array {
        count: 3,
        value_type: Box::new(LLVMType::I8),
      }),
      value: r#"c"%d\00""#.to_owned(),
    }
  }
}

#[derive(Clone, Debug)]
pub struct LocalVariable {
  pub name: String,
  pub value_type: LLVMType,
  pub value: String,
}

impl LocalVariable {
  pub fn new(name: String, value_type: LLVMType, value: String) -> Self {
    Self {
      name,
      value_type,
      value,
    }
  }
}

#[derive(Clone, Debug)]
pub struct Array {
  pub count: i32,
  pub value_type: Box<LLVMType>,
}

impl Array {
  pub fn new(count: i32, value_type: Box<LLVMType>) -> Self {
    Self { count, value_type }
  }
}

#[derive(Clone, Debug)]
pub enum LLVMType {
  I8Ptr,
  I32Ptr,
  I8,
  I32,
  Array(Array),
}

impl fmt::Display for LLVMType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      LLVMType::I8Ptr => write!(f, "i8*"),
      LLVMType::I32Ptr => write!(f, "i32*"),
      LLVMType::I8 => write!(f, "i8"),
      LLVMType::I32 => write!(f, "i32"),
      LLVMType::Array(arr) => write!(f, "[{} x {}]", arr.count, *arr.value_type),
    }
  }
}

#[derive(Clone, Debug)]
pub enum AccessModifier {
  Global,
  Private,
}

impl fmt::Display for AccessModifier {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      AccessModifier::Global => write!(f, "global"),
      AccessModifier::Private => write!(f, "private"),
    }
  }
}
