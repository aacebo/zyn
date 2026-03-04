use std::collections::HashSet;

use zyn::ItemFn;
use zyn::Render;
use zyn::proc_macro2::Ident;
use zyn::syn::BinOp;
use zyn::syn::Expr;
use zyn::syn::ExprAssign;
use zyn::syn::ExprBinary;
use zyn::syn::Local;
use zyn::syn::Pat;
use zyn::syn::Stmt;
use zyn::syn::fold;
use zyn::syn::fold::Fold;

use crate::AssignTrace;
use crate::LetTrace;

pub struct TraceVarFolder {
    pub input: ItemFn,
    pub vars: HashSet<Ident>,
}

struct TraceVarFolderInner {
    vars: HashSet<Ident>,
}

impl TraceVarFolderInner {
    fn is_traced_expr(&self, e: &Expr) -> bool {
        match e {
            Expr::Path(e) => {
                e.path.leading_colon.is_none() && e.path.segments.len() == 1 && {
                    let seg = e.path.segments.first().unwrap();
                    self.vars.contains(&seg.ident) && seg.arguments.is_empty()
                }
            }
            _ => false,
        }
    }

    fn is_traced_pat(&self, p: &Pat) -> bool {
        match p {
            Pat::Ident(p) => self.vars.contains(&p.ident),
            _ => false,
        }
    }
}

impl Fold for TraceVarFolderInner {
    fn fold_expr(&mut self, e: Expr) -> Expr {
        match e {
            Expr::Assign(ExprAssign {
                attrs,
                left,
                eq_token,
                right,
            }) => {
                let right = fold::fold_expr(self, *right);
                let left = *left;
                let op = zyn::zyn!({ { eq_token } });
                if self.is_traced_expr(&left) {
                    zyn::syn::parse2(AssignTrace { left, op, right }.render().unwrap()).unwrap()
                } else {
                    Expr::Assign(ExprAssign {
                        attrs,
                        left: Box::new(left),
                        eq_token,
                        right: Box::new(right),
                    })
                }
            }
            Expr::Binary(ref bin) if is_assign_op(bin.op) => {
                if let Expr::Binary(ExprBinary {
                    attrs,
                    left,
                    op,
                    right,
                }) = e.clone()
                {
                    let right = fold::fold_expr(self, *right);
                    let left = *left;
                    let op_ts = zyn::zyn!({ { op } });
                    if self.is_traced_expr(&left) {
                        zyn::syn::parse2(
                            AssignTrace {
                                left,
                                op: op_ts,
                                right,
                            }
                            .render()
                            .unwrap(),
                        )
                        .unwrap()
                    } else {
                        Expr::Binary(ExprBinary {
                            attrs,
                            left: Box::new(left),
                            op,
                            right: Box::new(right),
                        })
                    }
                } else {
                    unreachable!()
                }
            }
            _ => fold::fold_expr(self, e),
        }
    }

    fn fold_stmt(&mut self, s: Stmt) -> Stmt {
        match s {
            Stmt::Local(ref local) if local.init.is_some() && self.is_traced_pat(&local.pat) => {
                let Stmt::Local(local) = s else {
                    unreachable!()
                };
                let Local { pat, init, .. } = local;
                let init_expr = fold::fold_expr(self, *init.unwrap().expr);
                let ident = match &pat {
                    Pat::Ident(p) => p.ident.clone(),
                    _ => unreachable!(),
                };
                zyn::syn::parse2(
                    LetTrace {
                        pat,
                        init: init_expr,
                        ident,
                    }
                    .render()
                    .unwrap(),
                )
                .unwrap()
            }
            _ => fold::fold_stmt(self, s),
        }
    }
}

impl Render for TraceVarFolder {
    fn render(&self) -> zyn::Result {
        let mut folder = TraceVarFolderInner {
            vars: self.vars.clone(),
        };
        Ok(zyn::zyn!({ { folder.fold_item_fn(self.input.0.clone()) } }))
    }
}

fn is_assign_op(op: BinOp) -> bool {
    matches!(
        op,
        BinOp::AddAssign(_)
            | BinOp::SubAssign(_)
            | BinOp::MulAssign(_)
            | BinOp::DivAssign(_)
            | BinOp::RemAssign(_)
            | BinOp::BitXorAssign(_)
            | BinOp::BitAndAssign(_)
            | BinOp::BitOrAssign(_)
            | BinOp::ShlAssign(_)
            | BinOp::ShrAssign(_)
    )
}
