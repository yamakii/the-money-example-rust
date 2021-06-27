#![allow(dead_code)]

use std::marker::PhantomData;

trait Currency {
    fn currency() -> &'static str;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Dollar {}

impl Currency for Dollar {
    fn currency() -> &'static str { "USD" }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Franc {}

impl Currency for Franc {
    fn currency() -> &'static str { "CHF" }
}


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Money<T: Currency> {
    amount: i32,
    currency: PhantomData<T>,
}

impl<T: Currency> Money<T> {
    fn new(amount: i32) -> Self {
        Self { amount, currency: PhantomData::<T> }
    }
    fn times(self, multiplier: i32) -> Self {
        Money::new(self.amount * multiplier)
    }
    fn currency(self) -> &'static str {
        T::currency()
    }
}

#[cfg(test)]
mod tests {
    use crate::money::{Money, Dollar, Franc};

    #[test]
    fn test_currency() {
        assert_eq!("USD", Money::<Dollar>::new(1).currency());
        assert_eq!("CHF", Money::<Franc>::new(1).currency());
    }

    #[test]
    fn test_multiplication() {
        let five = Money::<Dollar>::new(5);
        assert_eq!(Money::<Dollar>::new(10), five.times(2));
        assert_eq!(Money::<Dollar>::new(15), five.times(3));
    }

    #[test]
    fn test_equality() {
        assert_ne!(Money::<Dollar>::new(5), Money::<Dollar>::new(6));
        assert_eq!(Money::<Dollar>::new(5), Money::<Dollar>::new(5));
    }
}