use std::ops::Deref;

use analyzer::expression_analyzer::analyze_expression;
use analyzer::expression_analyzer::get_adt_type;
use analyzer::function_analyzer::analyze_function;
use analyzer::static_env::StaticEnv;
use ast::Definition;
use ast::Expr;
use ast::Pattern;
use ast::Type;
use errors::*;
use types::Function;
use types::Value;
use util::StringConversion;

pub mod static_env;
pub mod module_analyser;
mod function_analyzer;
mod expression_analyzer;
mod dependency_sorter;
mod pattern_analyzer;
mod type_helper;

#[derive(Clone, Debug, PartialEq)]
pub enum PatternMatchingError {
    ListPatternsAreNotHomogeneous(Type, Type),
    UnknownOperatorPattern(String),
    UnknownAdtVariant(String),
    ExpectedListType(Type),
    ExpectedUnit(Type),
    ExpectedTuple(Pattern, Type),
    ExpectedRecord(Type),
    ExpectedAdt(String, Type),
    PatternNotExhaustive(Pattern),
    InvalidRecordEntryName(String),
    ExpectedLiteral(String, Type),
    TODO,
}


pub fn type_check_expression(env: &mut StaticEnv, expr: &Expr) -> Result<Type, TypeError> {
    analyze_expression(env, None, expr)
}

pub fn type_check_function(env: &mut StaticEnv, fun: &Definition) -> Result<Type, TypeError> {
    analyze_function(env, fun)
}


pub fn type_of_value(value: &Value) -> Type {
    match value {
        Value::Unit => {
            Type::Unit
        }
        Value::Number(_) => {
            Type::Var("number".s())
        }
        Value::Int(_) => {
            Type::Tag("Int".s(), vec![])
        }
        Value::Float(_) => {
            Type::Tag("Float".s(), vec![])
        }
        Value::String(_) => {
            Type::Tag("String".s(), vec![])
        }
        Value::Char(_) => {
            Type::Tag("Char".s(), vec![])
        }
        Value::List(items) => {
            if items.is_empty() {
                Type::Tag("List".s(), vec![Type::Var("a".s())])
            } else {
                Type::Tag("List".s(), vec![type_of_value(items.first().unwrap())])
            }
        }
        Value::Tuple(items) => {
            Type::Tuple(items.iter().map(|i| type_of_value(i)).collect())
        }
        Value::Record(items) => {
            Type::Record(items.iter().map(|(s, i)| (s.to_owned(), type_of_value(i))).collect())
        }
        Value::Adt(var_name, items, adt) => {
            get_adt_type(var_name, items, adt.clone())
        }
        Value::Fun { fun, args, .. } => {
            let fun_ty = type_of_function(fun.deref());

            strip_fun_args(args.len(), &fun_ty).clone()
        }
    }
}

fn type_of_function(fun: &Function) -> &Type {
    match fun {
        Function::External(_, _, ty) => ty,
        Function::Wrapper(_, _, ty) => ty,
        Function::Expr(_, _, _, ty) => ty,
    }
}

fn strip_fun_args(args: usize, ty: &Type) -> &Type {
    if args == 0 {
        return ty;
    }

    if let Type::Fun(_, ref output) = ty {
        strip_fun_args(args - 1, output)
    } else {
        ty
    }
}


#[cfg(test)]
mod tests {

    #[test]
    #[ignore]
    fn type_check1() {
        // TODO
//        let ast = from_code_mod(include_bytes!("../../benches/data/type_check.elm"));
//        let info = InterModuleInfo::new();

//        let module_info = ModuleInfo {
//            path: vec![],
//            ast,
//            code: String::from(include_str!("../../benches/data/type_check.elm")),
//        };


//        let checked = analyze_module(&info, module_info).expect("Type error");
//        println!("{:?}", checked);
//        panic!();
    }
}