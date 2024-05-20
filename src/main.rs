use std::sync::Arc;

use swc::Compiler;
use swc_ecma_ast::{CallExpr, Callee, Expr, Ident};

fn main() {
    build_js()
}

fn build_js() {
    let sourcemap = Arc::new(Default::default());
    let compiler = Compiler::new(sourcemap);

    let node = Box::new(CallExpr {
        span: Default::default(),
        callee: Callee::Expr(Box::new(Expr::Ident(Ident {
            span: Default::default(),
            sym: "console.log".into(),
            optional: false,
        }))),
        args: Default::default(),
        type_args: Default::default(),
    });

    let output = compiler.print(&node, Default::default()).unwrap();

    if let Some(map) = output.map {
        println!("{}", map)
    };
    println!("{}", output.code);
}
