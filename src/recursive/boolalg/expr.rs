#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum ExprMatcher<T, Expr> {
    Var(T),
    Not(Expr),
    And(Expr, Expr),
    Or(Expr, Expr),
}

trait Expr: Clone {
    type T;
    fn new(matcher: ExprMatcher<Self::T, Self>) -> Self;
    fn matcher(&self) -> &ExprMatcher<Self::T, Self>;

    fn var(var: Self::T) -> Self {
        Self::new(ExprMatcher::Var(var))
    }

    fn not(&self) -> Self {
        Self::new(ExprMatcher::Not(self.clone()))
    }

    fn and(&self, other: &Self) -> Self {
        Self::new(ExprMatcher::And(self.clone(), other.clone()))
    }

    fn or(&self, other: &Self) -> Self {
        Self::new(ExprMatcher::Or(self.clone(), other.clone()))
    }
}

#[cfg(test)]
mod rc_tests {
    use super::{Expr, ExprMatcher};

    #[derive(Clone)]
    struct RcExpr(std::rc::Rc<ExprMatcher<isize, Self>>);

    impl Expr for RcExpr {
        type T = isize;

        fn new(matcher: ExprMatcher<Self::T, Self>) -> Self {
            Self(std::rc::Rc::new(matcher))
        }

        fn matcher(&self) -> &ExprMatcher<Self::T, Self> {
            &*self.0
        }
    }

    #[test]
    fn test_xx() {
        let v0 = RcExpr::var(0);
        let v1 = RcExpr::var(1);
        let _ = v0.and(&v1);
    }
}
