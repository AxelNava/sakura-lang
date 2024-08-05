#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_large_input() {
        let expected_result = 354224848179261915075;
        let input = 100;
        assert_eq!(fibbonacci(input), expected_result);
    }
}