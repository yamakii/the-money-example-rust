#![allow(dead_code)]

use std::marker::PhantomData;
use std::ops::Add;

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

enum Expression<T: Currency> {
    Sum(Sum<T>),
    Money(Money<T>),
}

impl<T: Currency> Expression<T> {
    fn reduce(self) -> Money<T> {
        match self {
            Expression::Sum(sum) => sum.reduce(),
            Expression::Money(money) => money,
        }
    }
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

impl<T: Currency> Add for Money<T> {
    type Output = Expression<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Expression::Sum(Sum::new(self, rhs))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Bank {}

impl Bank {
    fn reduce<T: Currency>(self, source: Expression<T>) -> Money<T> {
        source.reduce()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Sum<T: Currency> {
    augend: Money<T>,
    addend: Money<T>,
}

impl<T: Currency> Sum<T> {
    fn new(augend: Money<T>, addend: Money<T>) -> Self {
        Sum { augend, addend }
    }
    fn reduce(self) -> Money<T> {
        Money::new(self.augend.amount + self.addend.amount)
    }
}

#[cfg(test)]
mod tests {
    use crate::money::{Money, Dollar, Franc, Bank, Sum, Expression};

    #[test]
    fn test_currency() {
        assert_eq!("USD", Money::<Dollar>::new(1).currency());
        assert_eq!("CHF", Money::<Franc>::new(1).currency());
    }

    #[test]
    fn test_simple_addition() {
        let five = Money::<Dollar>::new(5);
        let sum = five + five;
        let bank = Bank {};
        let reduced = bank.reduce::<Dollar>(sum);
        assert_eq!(Money::<Dollar>::new(10), reduced);
    }

    #[test]
    fn test_plus_return_sum() {
        let five = Money::<Dollar>::new(5);
        let result = five + five;
        let sum = match result {
            Expression::Sum(x) => x,
            _ => panic!("Sumが来るはず")
        };
        assert_eq!(five, sum.augend);
        assert_eq!(five, sum.addend);
    }

    #[test]
    fn test_plus_reduce_sum() {
        let sum = Expression::<Dollar>::Sum(
            Sum::new(Money::new(3), Money::new(4))
        );
        let bank = Bank {};
        let reduced = bank.reduce::<Dollar>(sum);
        assert_eq!(Money::<Dollar>::new(7), reduced);
    }

    #[test]
    fn test_plus_reduce_money() {
        let bank = Bank {};
        let money = Expression::<Dollar>::Money(Money::new(1));
        let reduced = bank.reduce::<Dollar>(money);
        assert_eq!(Money::<Dollar>::new(1), reduced);
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