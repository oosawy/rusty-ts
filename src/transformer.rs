use swc_core::common::BytePos;
use swc_ecma_ast::{
    BlockStmt, CallExpr, Callee, Expr, ExprOrSpread, ExprStmt, FnDecl, Function, Ident, Lit, Stmt,
    Str,
};
use syn::{parse::Parser, spanned::Spanned};

pub struct Transformer();

impl Transformer {
    pub fn new() -> Self {
        Self {}
    }

    pub fn item_fn(&self, item_fn: &syn::ItemFn) -> FnDecl {
        FnDecl {
            ident: convert_ident(item_fn.sig.ident.clone()),
            declare: false,
            function: Box::new(Function {
                span: Default::default(),
                params: Default::default(),
                decorators: Default::default(),
                body: Some(self.block(&item_fn.block)),
                is_generator: false,
                is_async: false,
                type_params: Default::default(),
                return_type: Default::default(),
            }),
        }
    }

    pub fn block(&self, block: &syn::Block) -> BlockStmt {
        BlockStmt {
            span: Default::default(),
            stmts: block.stmts.iter().map(|stmt| self.stmt(stmt)).collect(),
        }
    }

    pub fn stmt(&self, stmt: &syn::Stmt) -> Stmt {
        match stmt {
            syn::Stmt::Local(_) => todo!(),
            syn::Stmt::Item(_) => todo!(),
            syn::Stmt::Expr(expr, _) => Stmt::Expr(ExprStmt {
                span: Default::default(),
                expr: Box::new(self.expr(expr)),
            }),
            syn::Stmt::Macro(macro_) => {
                // println!("hello world");
                if macro_.mac.path.is_ident("println".into()) {
                    let parser = syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_separated_nonempty;
                    // let _ = parserparse2::<parser>(macro_.mac.path.tokens.clone());
                    let args = parser.parse2(macro_.mac.tokens.clone()).unwrap();

                    return Stmt::Expr(ExprStmt {
                        span: convert_span(macro_.span()),
                        expr: Box::new(Expr::Call(CallExpr {
                            span: convert_span(macro_.mac.path.span().join(args.span()).unwrap()),
                            callee: Callee::Expr(Box::new(Expr::Ident({
                                Ident {
                                    span: Default::default(),
                                    sym: "console.log".into(),
                                    optional: false,
                                }
                            }))),
                            args: args
                                .into_iter()
                                .map(|expr| ExprOrSpread {
                                    expr: Box::new(self.expr(&expr)),
                                    spread: None,
                                })
                                .collect(),
                            type_args: None,
                        })),
                    });
                }

                todo!();
            }
        }
    }

    pub fn expr(&self, expr: &syn::Expr) -> Expr {
        match expr {
            syn::Expr::Array(_) => todo!(),
            syn::Expr::Assign(_) => todo!(),
            syn::Expr::Async(_) => todo!(),
            syn::Expr::Await(_) => todo!(),
            syn::Expr::Binary(_) => todo!(),
            syn::Expr::Block(_) => todo!(),
            syn::Expr::Break(_) => todo!(),
            syn::Expr::Call(_) => todo!(),
            syn::Expr::Cast(_) => todo!(),
            syn::Expr::Closure(_) => todo!(),
            syn::Expr::Const(_) => todo!(),
            syn::Expr::Continue(_) => todo!(),
            syn::Expr::Field(_) => todo!(),
            syn::Expr::ForLoop(_) => todo!(),
            syn::Expr::Group(_) => todo!(),
            syn::Expr::If(_) => todo!(),
            syn::Expr::Index(_) => todo!(),
            syn::Expr::Infer(_) => todo!(),
            syn::Expr::Let(_) => todo!(),
            syn::Expr::Lit(lit) => Expr::Lit(self.lit(&lit.lit)),
            syn::Expr::Loop(_) => todo!(),
            syn::Expr::Macro(_) => todo!(),
            syn::Expr::Match(_) => todo!(),
            syn::Expr::MethodCall(_) => todo!(),
            syn::Expr::Paren(_) => todo!(),
            syn::Expr::Path(_) => todo!(),
            syn::Expr::Range(_) => todo!(),
            syn::Expr::Reference(_) => todo!(),
            syn::Expr::Repeat(_) => todo!(),
            syn::Expr::Return(_) => todo!(),
            syn::Expr::Struct(_) => todo!(),
            syn::Expr::Try(_) => todo!(),
            syn::Expr::TryBlock(_) => todo!(),
            syn::Expr::Tuple(_) => todo!(),
            syn::Expr::Unary(_) => todo!(),
            syn::Expr::Unsafe(_) => todo!(),
            syn::Expr::Verbatim(_) => todo!(),
            syn::Expr::While(_) => todo!(),
            syn::Expr::Yield(_) => todo!(),
            _ => todo!(),
        }
    }

    pub fn lit(&self, lit: &syn::Lit) -> Lit {
        match lit {
            syn::Lit::Str(str) => Lit::Str(Str {
                span: convert_span(str.span()),
                value: str.value().into(),
                raw: None,
            }),
            syn::Lit::ByteStr(_) => todo!(),
            syn::Lit::CStr(_) => todo!(),
            syn::Lit::Byte(_) => todo!(),
            syn::Lit::Char(_) => todo!(),
            syn::Lit::Int(_) => todo!(),
            syn::Lit::Float(_) => todo!(),
            syn::Lit::Bool(_) => todo!(),
            syn::Lit::Verbatim(_) => todo!(),
            _ => todo!(),
        }
    }
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

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use swc::Compiler;
    use swc_core::common::Span;

    use super::*;

    #[test]
    fn test_lit() {
        let transformer = Transformer {};

        let lit = syn::parse_str::<syn::Lit>(r#""Hello, World!""#).unwrap();

        let result = transformer.lit(&lit);

        assert_eq!(
            result,
            Lit::Str(Str {
                span: Span::new(BytePos(0), BytePos(15), Default::default()),
                value: "Hello, World!".into(),
                raw: None
            })
        );
    }

    #[test]
    fn test_item_fn() {
        let transformer = Transformer {};

        let item_fn = syn::parse_str::<syn::ItemFn>(
            r#"fn hello_world() {
                println!("Hello, {}!", "world");
            }"#,
        )
        .unwrap();

        let result = transformer.item_fn(&item_fn);

        let code = Compiler::new(Arc::new(Default::default()))
            .print(
                &result,
                swc::PrintArgs {
                    source_map: swc_compiler_base::SourceMapsConfig::Bool(false),
                    ..Default::default()
                },
            )
            .unwrap()
            .code;

        assert_eq!(
            code,
            r#"function hello_world() {
    console.log("Hello, {}!", "world");
}"#
        );

        assert_eq!(
            result,
            FnDecl {
                ident: Ident {
                    span: Default::default(),
                    sym: "hello_world".into(),
                    optional: false
                },
                declare: false,
                function: Box::new(Function {
                    span: Default::default(),
                    params: Default::default(),
                    decorators: Default::default(),
                    body: Some(BlockStmt {
                        span: Default::default(),
                        stmts: vec![Stmt::Expr(ExprStmt {
                            span: Span::new(BytePos(35), BytePos(67), Default::default()),
                            expr: Box::new(Expr::Call(CallExpr {
                                span: Span::new(BytePos(35), BytePos(65), Default::default()),
                                callee: Callee::Expr(Box::new(Expr::Ident(Ident {
                                    span: Default::default(),
                                    sym: "console.log".into(),
                                    optional: false
                                }))),
                                args: vec![
                                    ExprOrSpread {
                                        expr: Box::new(Expr::Lit(Lit::Str(Str {
                                            span: Span::new(
                                                BytePos(44),
                                                BytePos(56),
                                                Default::default()
                                            ),
                                            value: "Hello, {}!".into(),
                                            raw: None
                                        }))),
                                        spread: None
                                    },
                                    ExprOrSpread {
                                        expr: Box::new(Expr::Lit(Lit::Str(Str {
                                            span: Span::new(
                                                BytePos(58),
                                                BytePos(65),
                                                Default::default()
                                            ),
                                            value: "world".into(),
                                            raw: None
                                        }))),
                                        spread: None
                                    }
                                ],
                                type_args: None
                            }))
                        })]
                    }),
                    is_generator: false,
                    is_async: false,
                    type_params: Default::default(),
                    return_type: Default::default()
                })
            }
        );
    }
}
