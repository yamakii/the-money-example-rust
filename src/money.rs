#![allow(dead_code)]

struct Dollar {
    amount: i32,
}

impl Dollar {
    fn new(amount: i32) -> Self {
        Dollar { amount }
    }
    fn times(&self, multiplier: i32) -> Self {
        Dollar::new(self.amount * multiplier)
    }
}

#[cfg(test)]
mod tests {
    use crate::money::Dollar;

    #[test]
    fn test_multiplication() {
        let five = Dollar::new(5);
        let mut product = five.times(2);
        assert_eq!(10, product.amount);
        product = five.times(3);
        assert_eq!(15, product.amount);
    }
}