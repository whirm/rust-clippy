use clippy_utils::diagnostics::{span_lint, span_lint_and_sugg};
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
    fn check_ty<'tcx>(&mut self, cx: &LateContext<'tcx>, hir_ty: &Ty<'tcx, AmbigArg>) {
        let ty = clippy_utils::ty::ty_from_hir_ty(cx, hir_ty.as_unambig_ty());

        if is_boxed_sized_ref(cx, ty) {
            span_lint(cx, BOX_REF, hir_ty.span, "TODO: lint msg");
        }
    }

    fn check_expr(&mut self, cx: &LateContext<'_>, expr: &'_ Expr<'_>) {
        // Get type of `expr`
        let ty = cx.typeck_results().expr_ty(expr);

        if is_boxed_sized_ref(cx, ty) {
            if let ExprKind::Call(_, &[Expr { span, .. }]) = expr.kind {
                span_lint_and_sugg(
                    cx,
                    BOX_REF,
                    expr.span,
                    "TODO: lint msg2",
                    "Remove Box",
                    format!("{}", snippet(cx, span, "<default>")),
                    Applicability::MachineApplicable,
                );
            };
        }
    }
}

fn is_boxed_sized_ref<'tcx>(cx: &LateContext<'tcx>, ty: rustc_middle::ty::Ty<'tcx>) -> bool {
    if let Some(boxed_ty) = ty.boxed_ty()
        && boxed_ty.is_ref()
    {
        boxed_ty.peel_refs().is_sized(cx.tcx, cx.typing_env())
    } else {
        false
    }
}
