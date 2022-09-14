pub type CostType = i64;
pub trait CostFunction {
    type Type;
    fn f(self, x: Self::Type) -> Option<Self::Type>;
}

#[derive(Debug, Clone, Copy)]
pub struct ConvexFunction {
    // y = a * x^2 + b * x + c
    a: CostType,
    b: CostType,
    c: CostType,
}

impl ConvexFunction {
    pub fn new(a: CostType, b: CostType, c: CostType) -> Self {
        ConvexFunction { a, b, c }
    }
}

impl CostFunction for ConvexFunction {
    type Type = CostType;
    fn f(self, x: Self::Type) -> Option<Self::Type> {
        Some(self.a * x * x + self.b * x + self.c)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct QuarticFunction {
    // y = a * x^4 + b * x^3 + c * x^2 + d * x + e
    a: CostType,
    b: CostType,
    c: CostType,
    d: CostType,
    e: CostType,
}

impl QuarticFunction {
    pub fn new(a: CostType, b: CostType, c: CostType, d: CostType, e: CostType) -> Self {
        QuarticFunction { a, b, c, d, e }
    }
}

impl CostFunction for QuarticFunction {
    type Type = CostType;
    fn f(self, x: Self::Type) -> Option<Self::Type> {
        Some(self.a * x * x * x * x + self.b * x * x * x + self.c * x * x + self.d * x + self.e)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convex_test() {
        let cf = ConvexFunction::new(1, -6, 9);
        assert_eq!(cf.f(0).unwrap(), 9);
        assert_eq!(cf.f(3).unwrap(), 0);
        assert_eq!(cf.f(-3).unwrap(), 36);
    }

    #[test]
    fn quartic_test() {
        let cf = QuarticFunction::new(3, -4, -180, 0, 0);
        assert_eq!(cf.f(-5).unwrap(), -2125);
        assert_eq!(cf.f(0).unwrap(), 0);
        assert_eq!(cf.f(6).unwrap(), -3456);
    }
}
