mod transformer;

use std::sync::Arc;

use swc::{atoms::Atom, Compiler};
use swc_core::common::BytePos;
use swc_ecma_ast::{
    CallExpr, Callee, Decl, Expr, ExprOrSpread, ExprStmt, Ident, Lit, Module, ModuleItem, Stmt, Str,
};
use syn::spanned::Spanned;
use transformer::Transformer;

fn main() {
    compile(r#"println!("hello world"); fn foo() { println!("foo") }"#)
}

fn compile(input: &str) {
    let file = match syn::parse_file(input) {
        Ok(parse) => parse,
        Err(error) => {
            println!("Failed to Parse input: {:?}", error);
            return;
        }
    };

    let output = generate(traverse(file));

    println!("{}", output);
}

fn traverse(
    input: syn::File,
) -> impl swc_ecma_codegen::Node + swc_ecma_visit::VisitWith<swc_compiler_base::IdentCollector> {
    let transformer = Transformer::new();

    let span = convert_span(input.span());

    let mut items: Vec<ModuleItem> = Vec::new();

    for item in input.items {
        match item {
            syn::Item::Macro(macro_) => {
                // println!()
                if macro_
                    .mac
                    .path
                    .segments
                    .last()
                    .is_some_and(|x| x.ident == "println")
                {
                    let string = syn::parse2::<syn::LitStr>(macro_.mac.tokens.clone())
                        .unwrap()
                        .value();

                    let expr = CallExpr {
                        span: convert_span(macro_.span()),
                        callee: Callee::Expr(Box::new(Expr::Ident(Ident {
                            span: Default::default(),
                            sym: "console.log".into(),
                            optional: false,
                        }))),
                        args: vec![ExprOrSpread {
                            expr: Box::new(Expr::Lit(Lit::Str(Str::from(string)))),
                            spread: None,
                        }],
                        // args: macro_
                        //     .mac
                        //     .tokens
                        //     .iter()
                        //     .map(|attr| ExprOrSpread {
                        //         expr: Box::new(Expr::Lit(Lit::Str(Str::from("()")))),
                        //         spread: None,
                        //     })
                        //     .collect::<Vec<ExprOrSpread>>(),
                        type_args: Default::default(),
                    };

                    let stmt = Stmt::Expr(ExprStmt {
                        span: Default::default(),
                        expr: expr.into(),
                    });

                    items.push(ModuleItem::Stmt(stmt));
                }
            }

            syn::Item::Fn(func) => {
                items.push(ModuleItem::Stmt(Stmt::Decl(Decl::Fn(
                    transformer.item_fn(&func),
                ))));

                // let stmt = Stmt::Decl(Decl::Fn(FnDecl {
                //     ident: convert_ident(func.sig.ident),
                //     declare: false,
                //     function: Box::new(Function {
                //         span: Default::default(),
                //         params: Default::default(),
                //         decorators: Default::default(),
                //         body: Default::default(),
                //         is_generator: false,
                //         is_async: false,
                //         type_params: Default::default(),
                //         return_type: Default::default(),
                //     }),
                // }));

                // items.push(ModuleItem::Stmt(stmt));
            }
            _ => {}
        }
    }

    return Module {
        span: span,
        body: items,
        shebang: input.shebang.map(|x| Atom::from(x)),
    };
}

fn convert_span(span: proc_macro2::Span) -> swc_core::common::Span {
    let range = span.byte_range();
    swc_core::common::Span::new(
        BytePos(range.start as u32),
        BytePos(range.end as u32),
        Default::default(),
    )
}

fn convert_ident(ident: syn::Ident) -> Ident {
    Ident {
        span: Default::default(),
        sym: ident.to_string().into(),
        optional: false,
    }
}

fn generate<
    T: swc_ecma_codegen::Node + swc_ecma_visit::VisitWith<swc_compiler_base::IdentCollector>,
>(
    node: T,
) -> String {
    let sourcemap = Arc::new(Default::default());
    let compiler = Compiler::new(sourcemap);

    let output = compiler.print(&node, Default::default()).unwrap();

    if let Some(map) = output.map {
        println!("{}", map)
    };
    return output.code;
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_build_js() {
//         compile()
//     }
// }
