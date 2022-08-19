// use crate::code_gen::llvm_context::LocalVariable;

// use super::{
//   llvm_context::AccessModifier,
//   llvm_context::LLVMType,
//   llvm_context::VarArgFunction,
//   llvm_context::{self, GlobalVariable},
//   llvm_module::LLVMProgramBuilder,
// };

// pub fn main() {
//   let printf = VarArgFunction::new(LLVMType::I32, "printf".to_owned(), vec![LLVMType::I8Ptr]);

//   let format_num = GlobalVariable::new(
//     "format_num".to_owned(),
//     AccessModifier::Private,
//     true,
//     LLVMType::Array(llvm_context::Array::new(3, Box::new(LLVMType::I8))),
//     r#"c"%d\00""#.to_owned(),
//   );

//   let v1 = LocalVariable::new("%1".to_owned(), LLVMType::I32, "1".to_owned());
//   let v2 = "%2".to_owned();

//   let prog = LLVMProgramBuilder::default()
//     .top_level_declare_var_arg_function(printf)
//     .top_level_declare_global_variable(format_num)
//     .declare_local_variable(v1.clone())
//     .load_local_variable(v2.to_owned(), v1)
//     .print_number(v2)
//     .return_ok()
//     .build();

//   match prog {
//     Ok(prog) => println!("{}", prog),
//     Err(err) => println!("{:?}", err),
//   }
// }
