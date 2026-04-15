// Tests for Problem 3387: Maximize Amount After Two Days of Conversions
// Java reference: src/test/java/g3301_3400/s3387_maximize_amount_after_two_days_of_conversions/SolutionTest.java

use leetcode_in_rust::s3387::maximize_amount_after_two_days_of_conversions::Solution;

#[test]
fn test_max_amount() {
    let result = Solution::max_amount(
        "EUR".to_string(),
        vec![vec!["EUR".to_string(), "USD".to_string()], vec!["USD".to_string(), "JPY".to_string()]],
        vec![2.0, 3.0],
        vec![vec!["JPY".to_string(), "USD".to_string()], vec!["USD".to_string(), "CHF".to_string()], vec!["CHF".to_string(), "EUR".to_string()]],
        vec![4.0, 5.0, 6.0],
    );
    assert!((result - 720.0).abs() < 0.001);
}

#[test]
fn test_max_amount2() {
    let result = Solution::max_amount(
        "NGN".to_string(),
        vec![vec!["NGN".to_string(), "EUR".to_string()]],
        vec![9.0],
        vec![vec!["NGN".to_string(), "EUR".to_string()]],
        vec![6.0],
    );
    assert!((result - 1.5).abs() < 0.001);
}

#[test]
fn test_max_amount3() {
    let result = Solution::max_amount(
        "USD".to_string(),
        vec![vec!["USD".to_string(), "EUR".to_string()]],
        vec![1.0],
        vec![vec!["EUR".to_string(), "JPY".to_string()]],
        vec![10.0],
    );
    assert!((result - 1.0).abs() < 0.001);
}
