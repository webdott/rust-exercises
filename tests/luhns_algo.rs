fn luhn_algorithm(n: u64) -> bool {
    let n_string = n.to_string();
    let length = n_string.len();
    let mut sum = 0;
    let parity = length % 2;

    for i in (0..(length - 1)).rev() {
        let card_number = &n_string[i..i+1].parse::<i32>().unwrap();
        let mut d = 0;

        if  i % 2 == parity {
            d = 2 * *card_number;

            if d > 9 {
                d -= 9;
            }

            sum += d;
        } else {
            sum += *card_number;
        }
    }

    let last_num = &n_string[length - 1..length].parse::<i32>().unwrap();

    *last_num == ((10 - (sum % 10)) % 10)
}

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use super::luhn_algorithm;

    #[test]
    fn luhn_zero() {
        assert!(luhn_algorithm(0));
    }

    #[test]
    fn luhn_small_correct() {
        assert!(luhn_algorithm(18));
    }

    #[test]
    fn luhn_small_incorrect() {
        assert!(!luhn_algorithm(10));
    }

    #[test]
    fn luhn_correct() {
        assert!(luhn_algorithm(17893729974));
        assert!(luhn_algorithm(79927398713));
    }

    #[test]
    fn luhn_incorrect() {
        assert!(!luhn_algorithm(17893729975));
        assert!(!luhn_algorithm(17893729976));
        assert!(!luhn_algorithm(17893729977));
        assert!(!luhn_algorithm(123456));
    }
}