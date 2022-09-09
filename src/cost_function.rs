pub type CostType = i64;
pub trait CostFunction {
    type Type;
    fn f(self, x: Self::Type) -> Option<Self::Type>;
}

#[derive(Debug, Clone, Copy)]
pub struct ConvexFunction {
    // y = a * (x - p)^2 + q
    a: CostType,
    p: CostType,
    q: CostType,
}

impl ConvexFunction {
    pub fn new(a: CostType, p: CostType, q: CostType) -> Self {
        ConvexFunction { a, p, q }
    }
}

impl CostFunction for ConvexFunction {
    type Type = CostType;
    fn f(self, x: Self::Type) -> Option<Self::Type> {
        Some(self.a * (x - self.p) * (x - self.p) + self.q)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convex_test() {
        let cf = ConvexFunction::new(1, 20, 0);
        assert_eq!(cf.f(0).unwrap(), 400);
        assert_eq!(cf.f(20).unwrap(), 0);
        assert_eq!(cf.f(30).unwrap(), 100);
    }
}
