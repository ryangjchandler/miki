#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(parser);

mod prelude;
mod ast;

use ast::{Stmt, Expr, Block};
use parser::ProgramParser;

const PROGRAM: &'static str = r###"
def fib(n) {
    if n < 2 {
        return n
    }

    return fib(n - 1) + fib(n - 2)
}

println!("fib(30): {}", fib(30))
"###;

fn main() {
    // shell();

    let ast = ProgramParser::new().parse(PROGRAM).unwrap();

    let mut compiled = start_compiler();
    for stmt in ast.iter() {
        if ! matches!(stmt, Stmt::Def { .. }) {
            continue
        }

        compiled.push_str(&compile_statement(stmt));
    }
    compiled.push_str("\n\nfn main() {\n");
    // TODO: Segment definitions from main statements.
    for stmt in ast.iter() {
        if matches!(stmt, Stmt::Def { .. }) {
            continue
        }

        compiled.push_str(&compile_statement(stmt));
    }
    compiled.push_str("\n}");

    std::fs::write("output.rs", compiled).unwrap();
}

fn start_compiler() -> String {
    let mut s = String::new();
    s.push_str(include_str!("./prelude.rs"));
    s
}

fn compile_block(block: &Block) -> String {
    let mut b = String::new();
    for stmt in block {
        b.push_str(&compile_statement(stmt));
    }
    b
}

fn compile_statement(stmt: &Stmt) -> String {
    match stmt {
        Stmt::Def { name, params, body } => {
            let mut d = format!("fn {}(", name);
            let mut param_string = String::new();
            for (i, param) in params.iter().enumerate() {
                if i > 0 {
                    param_string.push_str(", ");
                }

                param_string.push_str(&format!("{}: Object", compile_expr(param)));
            }
            d.push_str(&param_string);
            d.push_str(") -> Object {\n");
            d.push_str(&compile_block(body));
            d.push_str("\n}");
            d
        },
        Stmt::If { cond, then } => {
            let mut i = String::from("if ");
            i.push_str(&compile_expr(cond));
            i.push_str("{\n");
            i.push_str(&compile_block(then));
            i.push_str("\n}");
            i
        },
        Stmt::Expr { expr } => compile_expr(expr),
        Stmt::Return { expr } => format!("return {};", compile_expr(expr)),
        _ => unreachable!("unrecognised stmt: {:?}", stmt),
    }
}

fn compile_expr(expr: &Expr) -> String {
    match expr {
        Expr::Str { s } => format!("\"{}\"", s),
        Expr::Ident { n } => n.to_string(),
        Expr::Int { i } => format!("Object::Int({})", i),
        Expr::LessThan { lhs, rhs } => format!("{}.clone() < {}.clone()", compile_expr(lhs), compile_expr(rhs)),
        Expr::Add { lhs, rhs } => format!("{}.clone() + {}.clone()", compile_expr(lhs), compile_expr(rhs)),
        Expr::Sub { lhs, rhs } => format!("{}.clone() - {}.clone()", compile_expr(lhs), compile_expr(rhs)),
        Expr::Call { n, args } => format!("{}({})", compile_expr(n), args.iter().map(|e| compile_expr(e)).collect::<Vec<String>>().join(", ")),
        _ => unreachable!("Unrecognised expr: {:?}", expr)
    }
}