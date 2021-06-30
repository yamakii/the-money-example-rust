#![allow(dead_code)]

use std::ops::Add;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Dollar;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Franc;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Currency {
    Dollar(Dollar),
    Franc(Franc),
}

impl Currency {
    fn currency(self) -> &'static str {
        match self {
            Currency::Dollar(_) => "USD",
            Currency::Franc(_) => "CHF"
        }
    }
}

enum Expression {
    Sum(Sum),
    Money(Money),
}

impl Expression {
    fn reduce(self, to: Currency) -> Money {
        match self {
            Expression::Sum(sum) => sum.reduce(to),
            Expression::Money(money) => money.reduce(to),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Money {
    amount: i32,
    currency: Currency,
}

impl Money {
    fn new(amount: i32, currency: Currency) -> Self {
        Self { amount, currency }
    }
    fn dollar(amount: i32) -> Self {
        Self { amount, currency: Currency::Dollar(Dollar) }
    }
    fn franc(amount: i32) -> Self {
        Self { amount, currency: Currency::Franc(Franc) }
    }
    fn times(self, multiplier: i32) -> Self {
        Money::new(self.amount * multiplier, self.currency)
    }
    fn reduce(self, to: Currency) -> Self {
        Money::new(self.amount, to)
    }
    fn currency(self) -> &'static str {
        self.currency.currency()
    }
}

impl Add for Money {
    type Output = Expression;

    fn add(self, rhs: Self) -> Self::Output {
        Expression::Sum(Sum::new(self, rhs))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Bank {}

impl Bank {
    fn reduce(self, source: Expression, to: Currency) -> Money {
        source.reduce(to)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Sum {
    augend: Money,
    addend: Money,
}

impl Sum {
    fn new(augend: Money, addend: Money) -> Self {
        Sum { augend, addend }
    }
    fn reduce(self, to: Currency) -> Money {
        Money::new(self.augend.amount + self.addend.amount, to)
    }
}

#[cfg(test)]
mod tests {
    use crate::money_unit::{Money, Bank, Sum, Expression, Currency, Dollar};

    #[test]
    fn test_currency() {
        assert_eq!("USD", Money::dollar(1).currency());
        assert_eq!("CHF", Money::franc(1).currency());
    }

    #[test]
    fn test_simple_addition() {
        let five = Money::dollar(5);
        let sum = five + five;
        let bank = Bank {};
        let reduced = bank.reduce(sum, Currency::Dollar(Dollar));
        assert_eq!(Money::dollar(10), reduced);
    }

    #[test]
    fn test_plus_return_sum() {
        let five = Money::dollar(5);
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
        let sum = Expression::Sum(
            Sum::new(Money::dollar(3), Money::dollar(4))
        );
        let bank = Bank {};
        let reduced = bank.reduce(sum, Currency::Dollar(Dollar));
        assert_eq!(Money::dollar(7), reduced);
    }

    #[test]
    fn test_plus_reduce_money() {
        let bank = Bank {};
        let money = Expression::Money(Money::dollar(1));
        let reduced = bank.reduce(money, Currency::Dollar(Dollar));
        assert_eq!(Money::dollar(1), reduced);
    }

    #[test]
    fn test_multiplication() {
        let five = Money::dollar(5);
        assert_eq!(Money::dollar(10), five.times(2));
        assert_eq!(Money::dollar(15), five.times(3));
    }

    #[test]
    fn test_equality() {
        assert_ne!(Money::dollar(5), Money::dollar(6));
        assert_eq!(Money::dollar(5), Money::dollar(5));
    }
}