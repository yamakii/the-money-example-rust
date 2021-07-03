#![allow(dead_code)]

use std::ops::{Add, Mul};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Dollar;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Franc;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Currency {
    Dollar(Dollar),
    Franc(Franc),
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

impl FromStr for Currency {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "USD" => Result::Ok(Dollar.into()),
            "CHF" => Result::Ok(Franc.into()),
            _ => Result::Err(())
        }
    }
}

impl ToString for Currency {
    fn to_string(&self) -> String {
        match self {
            Currency::Dollar(_) => "USD".to_string(),
            Currency::Franc(_) => "CHF".to_string(),
        }
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
    fn times(&self, multiplier: i32) -> Expression {
        match self {
            Expression::Sum(sum) => sum * multiplier,
            Expression::Money(money) => *money * multiplier,
        }
    }
}

impl Add for &Expression {
    type Output = Expression;

    fn add(self, rhs: Self) -> Self::Output {
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
    fn reduce(&self, bank: &Bank, to: Currency) -> Self {
        let rate = bank.rate(self.currency, to);
        Money::new(self.amount / rate, to)
    }
    fn currency(&self) -> Currency {
        self.currency
    }
}

impl Add for Money {
    type Output = Expression;

    fn add(self, rhs: Self) -> Self::Output {
        &Expression::Money(self) + &Expression::Money(rhs)
    }
}

impl Mul<i32> for Money {
    type Output = Expression;

    fn mul(self, rhs: i32) -> Self::Output {
        Money::new(self.amount * rhs, self.currency).into()
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

impl Mul<i32> for &Sum {
    type Output = Expression;

    fn mul(self, rhs: i32) -> Self::Output {
        Sum::new(self.augend.times(rhs), self.addend.times(rhs)).into()
    }
}

#[cfg(test)]
mod tests {
    use crate::money::{Money, Bank, Sum, Expression, Dollar, Franc, Currency};
    use std::str::FromStr;

    #[test]
    fn test_currency() {
        assert_eq!(Currency::Dollar(Dollar), Money::dollar(1).currency());
        assert_eq!(Currency::Franc(Franc), Money::franc(1).currency());
    }

    #[test]
    fn test_currency_from_str() {
        assert_eq!(Currency::Dollar(Dollar), Currency::from_str("USD").unwrap());
        assert_eq!(Currency::Franc(Franc), Currency::from_str("CHF").unwrap());
        assert_eq!(Result::<Currency, ()>::Err(()), Currency::from_str(""))
    }

    #[test]
    fn test_equality() {
        assert_ne!(Money::dollar(5), Money::dollar(6));
        assert_eq!(Money::dollar(5), Money::dollar(5));
        assert_ne!(Money::dollar(5), Money::franc(5));
    }

    #[test]
    fn test_multiplication() {
        let five: Expression = Money::dollar(5).into();
        assert_eq!(Expression::Money(Money::dollar(10)), five.times(2));
        assert_eq!(Expression::Money(Money::dollar(15)), five.times(3));
    }

    #[test]
    fn test_simple_addition() {
        let five: Expression = Money::dollar(5).into();
        let sum = &five + &five;
        let bank = Bank::new();
        let reduced = bank.reduce(sum, Dollar.into());
        assert_eq!(Money::dollar(10), reduced);
    }

    #[test]
    fn test_plus_return_sum() {
        let five: Expression = Money::dollar(5).into();
        let result = &five + &five;
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
        let result = bank.reduce(&five_bucks + &ten_francs, Dollar.into());
        assert_eq!(Money::dollar(10), result);
    }

    #[test]
    fn test_sum_times() {
        let five_bucks: Expression = Money::dollar(5).into();
        let ten_francs: Expression = Money::franc(10).into();
        let mut bank = Bank::new();
        bank.add_rate(Franc.into(), Dollar.into(), 2);
        let sum: Expression = (&Sum::new(five_bucks, ten_francs) * 2).into();
        let result = bank.reduce(sum, Dollar.into());
        assert_eq!(Money::dollar(20), result);
    }
}