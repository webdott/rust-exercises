//! Run this file with `cargo test --test 03_fibonacci`.

//! TODO: Implement a struct called `Fibonacci`, which implements `Iterator` that iterates through
//! Fibonacci numbers (starting from 0).
//! `Fibonacci` should implement the `Default` trait. 

struct Fibonacci {
    n: usize,
    fib_list: Vec<u64>
}

impl Default for Fibonacci {
    fn default() -> Self {
        Self { n: 0, fib_list: vec![0, 1] }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
       let result = self.nth(self.n)?;
       self.n += 1;
       Some(result)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if n < self.fib_list.len() { return Some(self.fib_list[n]) }

        let fib_val = self.nth(n - 1)? + self.nth(n - 2)?;
        self.fib_list.push(fib_val);
        Some(fib_val)
    }
}


/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use crate::Fibonacci;

    #[test]
    fn fibonacci_first() {
        assert_eq!(Fibonacci::default().next(), Some(0u64));
    }

    #[test]
    fn fibonacci_ten() {
        assert_eq!(
            Fibonacci::default().take(10).collect::<Vec<_>>(),
            vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]
        );
    }

    #[test]
    fn fibonacci_twenty() {
        assert_eq!(Fibonacci::default().nth(19), Some(4181));
    }

    #[test]
    fn fibonacci_sixty() {
        assert_eq!(Fibonacci::default().nth(59), Some(956722026041));
    }
}