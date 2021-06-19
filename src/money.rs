#![allow(dead_code)]

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Dollar {
    amount: i32,
}

impl Dollar {
    fn new(amount: i32) -> Self {
        Dollar { amount }
    }
    fn times(self, multiplier: i32) -> Self {
        Dollar::new(self.amount * multiplier)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Franc {
    amount: i32,
}

impl Franc {
    fn new(amount: i32) -> Self {
        Franc { amount }
    }
    fn times(self, multiplier: i32) -> Self {
        Franc::new(self.amount * multiplier)
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
    }
    #[test]
    fn test_franc_multiplication() {
        let five = Franc::new(5);
        assert_eq!(Franc::new(10), five.times(2));
        assert_eq!(Franc::new(15), five.times(3));
    }
}