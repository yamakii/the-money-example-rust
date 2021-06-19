#![allow(dead_code)]

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Money {
    amount: i32,
}

impl Money {
    fn new(amount: i32) -> Self {
        Money { amount }
    }
    fn times(self, multiplier: i32) -> Self {
        Money::new(self.amount * multiplier)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Dollar {
    amount: Money,
}

impl Dollar {
    fn new(amount: i32) -> Self {
        Dollar { amount: Money::new(amount) }
    }
    fn times(self, multiplier: i32) -> Self {
        Dollar { amount: self.amount.times(multiplier) }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Franc {
    amount: Money,
}

impl Franc {
    fn new(amount: i32) -> Self {
        Franc { amount: Money::new(amount) }
    }
    fn times(self, multiplier: i32) -> Self {
        Franc { amount: self.amount.times(multiplier) }
    }
}

#[cfg(test)]
mod tests {
    use crate::money::{Dollar, Franc};

    #[test]
    fn test_multiplication() {
        let five = Dollar::new(5);
        assert_eq!(Dollar::new(10), five.times(2));
        assert_eq!(Dollar::new(15), five.times(3));
    }

    #[test]
    fn test_equality() {
        assert_eq!(Dollar::new(5), Dollar::new(5));
        assert_ne!(Dollar::new(5), Dollar::new(6));
        assert_eq!(Franc::new(5), Franc::new(5));
        assert_ne!(Franc::new(5), Franc::new(6));
    }

    #[test]
    fn test_franc_multiplication() {
        let five = Franc::new(5);
        assert_eq!(Franc::new(10), five.times(2));
        assert_eq!(Franc::new(15), five.times(3));
    }
}