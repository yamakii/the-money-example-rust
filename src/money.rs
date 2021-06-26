#![allow(dead_code)]

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Money {
    amount: i32,
    currency: &'static str,
}


impl Money {
    fn new(amount: i32, currency: &'static str) -> Self {
        Money { amount, currency }
    }
    fn dollar(amount: i32) -> Self { Money::new(amount, "USD") }
    fn franc(amount: i32) -> Self { Money::new(amount, "CHF") }
    fn times(self, multiplier: i32) -> Self {
        Money::new(self.amount * multiplier, self.currency)
    }
    fn currency(self) -> &'static str { self.currency }
}

#[cfg(test)]
mod tests {
    use crate::money::{Money};

    #[test]
    fn test_currency() {
        assert_eq!("USD", Money::dollar(1).currency());
        assert_eq!("CHF", Money::franc(1).currency());
    }

    #[test]
    fn test_multiplication() {
        let five = Money::dollar(5);
        assert_eq!(Money::dollar(10), five.times(2));
        assert_eq!(Money::dollar(15), five.times(3));
    }

    #[test]
    fn test_equality() {
        assert_eq!(Money::dollar(5), Money::dollar(5));
        assert_ne!(Money::dollar(5), Money::dollar(6));
        assert_ne!(Money::dollar(5), Money::franc(5));
    }
}