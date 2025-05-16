use clippy_utils::diagnostics::span_lint_and_sugg;
use clippy_utils::source::snippet;
use rustc_errors::Applicability;
use rustc_hir::*;
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::declare_lint_pass;

declare_clippy_lint! {
    /// ### What it does
    ///
    /// ### Why is this bad?
    ///
    /// ### Example
    /// ```no_run
    /// // example code where clippy issues a warning
    /// ```
    /// Use instead:
    /// ```no_run
    /// // example code which does not raise clippy warning
    /// ```
    #[clippy::version = "1.88.0"]
    pub BOX_REF,
    nursery,
    "default lint description"
}

declare_lint_pass!(BoxRef => [BOX_REF]);

impl LateLintPass<'_> for BoxRef {
    //This works but doesn't span the whole expression
    // fn check_ty<'tcx>(&mut self, cx: &LateContext<'tcx>, ty: &Ty<'tcx, AmbigArg>) {
    //     let ty_n = clippy_utils::ty::ty_from_hir_ty(cx, ty.as_unambig_ty());
    //     if ty_n.boxed_ty().is_some_and(|x| x.is_ref()) {
    //         span_lint(cx, BOX_REF, ty.span, "Ooops");
    //     };
    // }
    fn check_expr(&mut self, cx: &LateContext<'_>, expr: &'_ rustc_hir::Expr<'_>) {
        // Get type of `expr`
        let ty = cx.typeck_results().expr_ty(expr);
        if let Some(boxed_ty) = ty.boxed_ty()
            && boxed_ty.is_ref()
        {
            let ref_span = if let ExprKind::Call(expr, &[Expr { hir_id, kind, span }]) = expr.kind {
                // dbg!(kind);
                span
            } else {
                todo!()
            };
            span_lint_and_sugg(
                cx,
                BOX_REF,
                expr.span,
                "This is nonsense! TODO",
                "Remove Box<>",
                format!("{}", snippet(cx, ref_span, "<default>")),
                Applicability::MachineApplicable,
            );
        }
        // // Match its kind to enter its type
        // if ty.boxed_ty().is_some_and(|x| x.is_ref()) {
        //     // span_lint(cx, BOX_REF, expr.span, "Ooops");
        // };
        // match ty.kind() {
        //     ty::Adt(adt_def, _) if adt_def.is_struct() => println!("Our `expr` is a struct!"),
        //     _ => (),
        // }
    }
}
