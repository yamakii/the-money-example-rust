struct Dollar {
    amount: i32,
}

impl Dollar {
    fn new(amount: i32) -> Self {
        Dollar { amount }
    }
    fn times(&mut self, multiplier: i32) {
        self.amount *= multiplier;
    }
}

#[cfg(test)]
mod tests {
    use crate::money::Dollar;

    #[test]
    fn test_multiplication() {
        let mut five = Dollar::new(5);
        five.times(2);
        assert_eq!(10, five.amount)
    }
}