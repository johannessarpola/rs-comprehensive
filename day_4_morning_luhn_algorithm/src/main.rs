pub fn luhn(cc_number: &str) -> bool {
    let mut sum = 0;
    let mut double = false;
    let mut digits = 0;

    for c in cc_number.chars().rev() {
        if c.is_whitespace() {
            continue;
        };

        let maybe_digit = c.to_digit(10);
        if maybe_digit.is_some() {
            digits += 1;
        }; 

        match maybe_digit {
            Some(digit) if double => {
                let doubled = digit * 2;
                sum += if doubled <= 9 { doubled } else { doubled - 9 } // basically 14 becomes 1 and 4
            }
            Some(digit) if !double => sum += digit,
            _ => return false,
        };
        double = !double; // flip switch
    }
    digits >= 2 && sum % 10 == 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_cc_number() {
        assert!(luhn("4263 9826 4026 9299"));
        assert!(luhn("4539 3195 0343 6467"));
        assert!(luhn("7992 7398 713"));
    }

    #[test]
    fn test_invalid_cc_number() {
        assert!(!luhn("4223 9826 4026 9299"));
        assert!(!luhn("4539 3195 0343 6476"));
        assert!(!luhn("8273 1232 7352 0569"));
    }
}

fn main() {
    let cc_number = "1234 5678 1234 5670";
    println!(
        "Is {cc_number} a valid credit card number? {}",
        if luhn(cc_number) { "yes" } else { "no" }
    );
}
