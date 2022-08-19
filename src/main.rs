#![deny(rust_2018_idioms)]
#![allow(dead_code)]
#![warn(clippy::unwrap_used)]

use lalrpop_util::{lexer::Token, ParseError};
use rust_sharp::{
  ast::SourcePos,
  bind::{binder::Binder, bound_ast::TypeError},
  code_gen::llvm_module::LLVMProgramBuilder,
  create_parser,
  parser::ProgramParser,
  print_error, print_program,
};
use std::io::Write;

fn main() {
  // code_gen::tmp::main();
  let parser = create_parser();

  // https://stackoverflow.com/a/34837038/12756474
  print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

  read_test(&parser);
  // read_repl(&parser);
}

/// Reads a hardcoded test file from the `samples/` folder.
fn read_test(parser: &ProgramParser) {
  let file_path = "samples/test.rsharp";
  let contents =
    std::fs::read_to_string(file_path).unwrap_or_else(|_| panic!("Unable to read {}", file_path));

  let mut binder = Binder::new();
  let mut llvm_builder = LLVMProgramBuilder::new();
  parse_input(parser, &contents, &mut binder, &mut llvm_builder);
}

/// Runs a Read Eval Print Loop. Enter an empty string to exit.
fn read_repl(parser: &ProgramParser) {
  let mut binder = Binder::new();
  let mut llvm_builder = LLVMProgramBuilder::new();

  loop {
    print!("> ");
    std::io::stdout().flush().ok();

    let mut line = String::new();
    match std::io::stdin().read_line(&mut line) {
      Ok(_) => {}
      Err(err) => panic!("{}", err),
    }

    if line.trim().is_empty() {
      break;
    }

    parse_input(parser, &line, &mut binder, &mut llvm_builder);
  }
}

#[allow(clippy::unwrap_used)]
fn parse_input(
  parser: &ProgramParser,
  input: &str,
  binder: &mut Binder,
  llvm_builder: &mut LLVMProgramBuilder,
) {
  // Parse
  let parsed_input = &parser.parse(input);
  if let Err(err) = parsed_input {
    handle_parse_error(input, err);
    return;
  }

  // Bind
  let bind_result = binder.bind(parsed_input.as_ref().unwrap());

  if let Err(errs) = bind_result {
    for err in errs.expr_errors {
      print_error(input, err.to_owned())
    }

    return;
  }

  let bind_result = bind_result.unwrap();
  print_program(&bind_result);

  // Compile
  llvm_builder.generate_llvm(bind_result);
}

fn handle_parse_error(line: &str, e: &ParseError<usize, Token<'_>, &str>) {
  match e {
    lalrpop_util::ParseError::UnrecognizedEOF {
      location,
      expected: _,
    } => {
      let msg = "Bad End Of File.".to_string();
      let err = TypeError {
        msg,
        source_pos: SourcePos {
          start: *location,
          end: location + 1,
        },
      };
      print_error(line, err)
    }
    lalrpop_util::ParseError::UnrecognizedToken { token, expected: _ } => {
      let msg = "Unrecognized Token.".to_string();
      let err = TypeError {
        msg,
        source_pos: SourcePos {
          start: token.0,
          end: token.2.to_owned(),
        },
      };
      print_error(line, err)
    }
    ParseError::InvalidToken { location } => {
      let msg = "Invalid Token.".to_string();
      let err = TypeError {
        msg,
        source_pos: SourcePos {
          start: location - 1,
          end: location.to_owned(),
        },
      };
      print_error(line, err)
    }
    _ => todo!(),
  }
}
