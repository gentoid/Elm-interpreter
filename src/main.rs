// Development only {
// cargo watch -s 'clear && cargo test'
#![allow(dead_code, unused_imports)]
// }

#[macro_use]
extern crate nom;
#[macro_use]
extern crate pretty_assertions;

use analyzer::environment::default_lang_env;
use analyzer::environment::Environment;
use analyzer::type_analyzer::get_type;
use interpreter::eval;
use nom::ExtendInto;
use nom::IResult;
use nom::verbose_errors::Context;
use parsers::expression::read_expr;
use parsers::module::*;
use parsers::statement::read_statement;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::Read;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use tokenizer::*;
use types::*;
use util::*;
use analyzer::environment::expand_env;

mod types;
#[macro_use]
mod util;
mod parsers;
mod tokenizer;
mod analyzer;
mod interpreter;

fn main() {
    interpret_stdin();
}

fn interpret_stdin() {
    print!("> ");
    stdout().flush().unwrap();
    let stdin = stdin();
    let mut env = default_lang_env();

    for line in stdin.lock().lines() {
        if let Err(s) = run_line(&mut env, &line.unwrap().as_bytes()) {
            println!("Error: {}", s);
        }
        print!("> ");
        stdout().flush().unwrap();
    }
}

fn run_line(env: &mut Environment, line: &[u8]) -> Result<(), String> {
    use nom::*;
    let tokens = get_all_tokens(line);

    let stm = read_statement(&tokens).map_err(|e| e.to_string());

    match stm {
        Ok((_, statement)) => {
            match statement {
                Statement::Alias(_path, _ty) => {}
                Statement::Adt(_def, _variants) => {}
                Statement::Port(_name, _ty) => {}
                Statement::Def(ref def) => {
                    expand_env(env, vec![def]).map_err(|e| format!("{:?}", e))?;
                }
            }
        }
        Err(_) => {
            let (_, expr) = read_expr(&tokens).map_err(|e| e.to_string())?;
            let expr_type = get_type(env, &expr).map_err(|e| format!("{:?}", e))?;
            env.enter_block();
            let value = eval(env, &expr);
            env.exit_block();

            println!("{} : {}", value?, expr_type);
        }
    }

    Ok(())
}

fn load_file() -> Vec<u8> {
    let mut file = File::open("example.elm").expect("Example file not found");
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data).unwrap();

    data
}

fn interpret_file() {
    let file = load_file();
    let tokens = get_all_tokens(&file);
//        println!("Tokens: \n{:#?}\n", tokens);

    let result = read_module(&tokens);

    if let Ok((rest, module)) = result {
        println!("Remaining: {:?}\n", rest);
        println!("Output: \n{:#?}", module);
    } else {
        println!("{:?}", result);
    }
}