fn main() {
    use Operation::*;
    let eq = ((Var * Var) - Val(1.0)) / ((Var * Var) + Val(1.0));
    let deq = eq.clone().symbolic_derivative();
    for x in -5..=5 {
        let result = eq.calc(x as f64);
        println!("{result}");
    }
    println!("\n");
    for x in -5..=5 {
        let result = deq.calc(x as f64);
        println!("{result}");
        assert!(result == eq.calc_derivative(x as f64))
    }
}
#[derive(Clone, Debug)]
enum Operation {
    Add(Box<Self>, Box<Self>),
    Mul(Box<Self>, Box<Self>),
    Div(Box<Self>, Box<Self>),
    Neg(Box<Self>),
    Val(f64),
    Var,
}
impl std::ops::Add for Operation {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.add(rhs)
    }
}
impl std::ops::Sub for Operation {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self.add(-rhs)
    }
}
impl std::ops::Mul for Operation {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        self.mul(rhs)
    }
}
impl std::ops::Div for Operation {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        self.div(rhs)
    }
}
impl std::ops::Neg for Operation {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self.neg()
    }
}
impl Operation {
    fn neg(self) -> Self {
        Self::Neg(Box::new(self))
    }
    fn add(self, other: Self) -> Self {
        Self::Add(Box::new(self), Box::new(other))
    }
    fn mul(self, other: Self) -> Self {
        Self::Mul(Box::new(self), Box::new(other))
    }
    fn div(self, other: Self) -> Self {
        Self::Div(Box::new(self), Box::new(other))
    }
    fn calc(&self, x: f64) -> f64 {
        match self {
            Self::Neg(inner) => -inner.calc(x),
            Self::Add(left, right) => left.calc(x) + right.calc(x),
            Self::Mul(left, right) => left.calc(x) * right.calc(x),
            Self::Div(left, right) => left.calc(x) / right.calc(x),
            Self::Val(value) => *value,
            Self::Var => x,
        }
    }

    fn symbolic_derivative(self) -> Self {
        match self {
            Self::Neg(inner) => -inner.symbolic_derivative(),
            Self::Add(left, right) => left.symbolic_derivative() + right.symbolic_derivative(),
            Self::Mul(left, right) => {
                let left = *left;
                let right = *right;
                left.clone() * right.clone().symbolic_derivative()
                    + left.symbolic_derivative() * right
            }
            Self::Div(left, right) => {
                let left = *left;
                let right = *right;
                let low_d_high = right.clone() * left.clone().symbolic_derivative();
                let high_d_low = left * right.clone().symbolic_derivative();
                let square_of_whats_below = right.clone() * right;

                (low_d_high - high_d_low) / square_of_whats_below
            }
            Self::Val(_) => Self::Val(0.0),
            Self::Var => Self::Val(1.0),
        }
    }
    fn calc_derivative(&self, x: f64) -> f64 {
        match self {
            Self::Neg(inner) => -inner.calc_derivative(x),
            Self::Add(left, right) => left.calc_derivative(x) + right.calc_derivative(x),
            Self::Mul(left, right) => {
                left.calc(x) * right.calc_derivative(x) + left.calc_derivative(x) * right.calc(x)
            }
            Self::Div(left, right) => {
                let low_d_high = right.calc(x) * left.calc_derivative(x);
                let high_d_low = left.calc(x) * right.calc_derivative(x);
                let square_of_whats_below = right.calc(x) * right.calc(x);

                (low_d_high - high_d_low) / square_of_whats_below
            }
            Self::Val(_) => 0.0,
            Self::Var => 1.0,
        }
    }
}
