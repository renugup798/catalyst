// Rust equivalent of simple_conversion.py

/// Calculate the nth Fibonacci number.
pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 | 1 => n,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

/// Sum all numbers in a vector.
pub fn sum_list(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}

/// Filter even numbers from a vector.
pub fn filter_even(numbers: &[i32]) -> Vec<i32> {
    numbers.iter().copied().filter(|n| n % 2 == 0).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(10), 55);
    }

    #[test]
    fn test_sum_list() {
        assert_eq!(sum_list(&[1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn test_filter_even() {
        assert_eq!(filter_even(&[1, 2, 3, 4, 5, 6]), vec![2, 4, 6]);
    }
}
