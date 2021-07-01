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
    fn currency(self) -> &'static str {
        match self {
            Currency::Dollar(_) => "USD",
            Currency::Franc(_) => "CHF"
        }
    }
}

impl From<Dollar> for Currency {
    fn from(dollar: Dollar) -> Self {
        Currency::Dollar(dollar)
    }
}

impl From<Franc> for Currency {
    fn from(franc: Franc) -> Self {
        Currency::Franc(franc)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Expression {
    Sum(Sum),
    Money(Money),
}

impl From<Money> for Expression {
    fn from(money: Money) -> Self {
        Expression::Money(money)
    }
}

impl From<Sum> for Expression {
    fn from(sum: Sum) -> Self {
        Expression::Sum(sum)
    }
}

impl Expression {
    fn reduce(&self, bank: &Bank, to: Currency) -> Money {
        match self {
            Expression::Sum(sum) => sum.reduce(bank, to),
            Expression::Money(money) => money.reduce(bank, to),
        }
    }
    fn plus(&self, rhs: &Self) -> Expression {
        Sum::new(self.clone(), rhs.clone()).into()
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
        Self { amount, currency: Dollar.into() }
    }
    fn franc(amount: i32) -> Self {
        Self { amount, currency: Franc.into() }
    }
    fn times(self, multiplier: i32) -> Expression {
        Money::new(self.amount * multiplier, self.currency).into()
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
        Sum::new(self.into(), rhs.into()).into()
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

#[derive(Clone, Debug, Eq, PartialEq)]
struct Sum {
    augend: Box<Expression>,
    addend: Box<Expression>,
}

impl Sum {
    fn new(augend: Expression, addend: Expression) -> Self {
        Sum {
            augend: Box::new(augend),
            addend: Box::new(addend),
        }
    }
    fn reduce(&self, bank: &Bank, to: Currency) -> Money {
        Money::new(
            self.augend.reduce(bank, to).amount + self.addend.reduce(bank, to).amount,
            to,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::money_unit::{Money, Bank, Sum, Expression, Dollar, Franc};

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
        assert_eq!(Expression::Money(Money::dollar(10)), five.times(2));
        assert_eq!(Expression::Money(Money::dollar(15)), five.times(3));
    }

    #[test]
    fn test_simple_addition() {
        let five = Money::dollar(5);
        let sum = five + five;
        let bank = Bank::new();
        let reduced = bank.reduce(sum, Dollar.into());
        assert_eq!(Money::dollar(10), reduced);
    }

    #[test]
    fn test_plus_return_sum() {
        let five: Expression = Money::dollar(5).into();
        let result = five.plus(&five);
        let sum = match result {
            Expression::Sum(x) => x,
            _ => panic!("Sumが来るはず")
        };
        assert_eq!(Box::new(five.clone()), sum.augend);
        assert_eq!(Box::new(five.clone()), sum.addend);
    }

    #[test]
    fn test_plus_reduce_sum() {
        let sum = Sum::new(
            Money::dollar(3).into(),
            Money::dollar(4).into(),
        ).into();
        let bank = Bank::new();
        let reduced = bank.reduce(sum, Dollar.into());
        assert_eq!(Money::dollar(7), reduced);
    }

    #[test]
    fn test_plus_reduce_money() {
        let bank = Bank::new();
        let money = Money::dollar(1).into();
        let reduced = bank.reduce(money, Dollar.into());
        assert_eq!(Money::dollar(1), reduced);
    }

    #[test]
    fn test_reduce_money_different_currency() {
        let mut bank = Bank::new();
        bank.add_rate(Franc.into(), Dollar.into(), 2);
        let result = bank.reduce(
            Money::franc(2).into(),
            Dollar.into(),
        );
        assert_eq!(Money::dollar(1), result);
    }

    #[test]
    fn test_identity_rate() {
        assert_eq!(1, Bank::new().rate(Dollar.into(), Dollar.into()));
    }

    #[test]
    fn test_mixed_addition() {
        let five_bucks: Expression = Money::dollar(5).into();
        let ten_francs: Expression = Money::franc(10).into();
        let mut bank = Bank::new();
        bank.add_rate(Franc.into(), Dollar.into(), 2);
        let result = bank.reduce(five_bucks.plus(&ten_francs), Dollar.into());
        assert_eq!(Money::dollar(10), result);
    }
}