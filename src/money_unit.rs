#![allow(dead_code)]

use std::ops::Add;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Dollar;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Franc;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Currency {
    Dollar(Dollar),
    Franc(Franc),
}

impl Currency {
    fn dollar() -> Self {
        Currency::Dollar(Dollar)
    }
    fn franc() -> Self {
        Currency::Franc(Franc)
    }
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
    fn reduce(self, bank: &Bank, to: Currency) -> Money {
        match self {
            Expression::Sum(sum) => sum.reduce(bank, to),
            Expression::Money(money) => money.reduce(bank, to),
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
        Self { amount, currency: Currency::dollar() }
    }
    fn franc(amount: i32) -> Self {
        Self { amount, currency: Currency::franc() }
    }
    fn times(self, multiplier: i32) -> Self {
        Money::new(self.amount * multiplier, self.currency)
    }
    fn reduce(self, bank: &Bank, to: Currency) -> Self {
        let rate = bank.rate(self.currency, to);
        Money::new(self.amount / rate, to)
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

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Pair(Currency, Currency);

#[derive(Debug)]
struct Bank {
    rates: HashMap<Pair, i32>,
}

impl Bank {
    fn new() -> Self {
        Bank { rates: HashMap::new() }
    }
    fn reduce(&self, source: Expression, to: Currency) -> Money {
        source.reduce(self, to)
    }
    fn add_rate(&mut self, from: Currency, to: Currency, rate: i32) {
        self.rates.insert(Pair(from, to), rate);
    }
    fn rate(&self, from: Currency, to: Currency) -> i32 {
        match self.rates.get(&Pair(from, to)) {
            None => { 1 }
            Some(rate) => { *rate }
        }
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
    fn reduce(self, _bank: &Bank, to: Currency) -> Money {
        Money::new(self.augend.amount + self.addend.amount, to)
    }
}

#[cfg(test)]
mod tests {
    use crate::money_unit::{Money, Bank, Sum, Expression, Currency};

    #[test]
    fn test_currency() {
        assert_eq!("USD", Money::dollar(1).currency());
        assert_eq!("CHF", Money::franc(1).currency());
    }

    #[test]
    fn test_equality() {
        assert_ne!(Money::dollar(5), Money::dollar(6));
        assert_eq!(Money::dollar(5), Money::dollar(5));
    }

    #[test]
    fn test_multiplication() {
        let five = Money::dollar(5);
        assert_eq!(Money::dollar(10), five.times(2));
        assert_eq!(Money::dollar(15), five.times(3));
    }

    #[test]
    fn test_simple_addition() {
        let five = Money::dollar(5);
        let sum = five + five;
        let bank = Bank::new();
        let reduced = bank.reduce(sum, Currency::dollar());
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
        let bank = Bank::new();
        let reduced = bank.reduce(sum, Currency::dollar());
        assert_eq!(Money::dollar(7), reduced);
    }

    #[test]
    fn test_plus_reduce_money() {
        let bank = Bank::new();
        let money = Expression::Money(Money::dollar(1));
        let reduced = bank.reduce(money, Currency::dollar());
        assert_eq!(Money::dollar(1), reduced);
    }

    #[test]
    fn test_reduce_money_different_currency() {
        let mut bank = Bank::new();
        bank.add_rate(Currency::franc(), Currency::dollar(), 2);
        let result = bank.reduce(
            Expression::Money(Money::franc(2)),
            Currency::dollar(),
        );
        assert_eq!(Money::dollar(1), result);
    }

    #[test]
    fn test_identity_rate() {
        assert_eq!(1, Bank::new().rate(Currency::dollar(), Currency::dollar()));
    }
}