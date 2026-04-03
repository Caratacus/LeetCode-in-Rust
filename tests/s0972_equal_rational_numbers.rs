// Tests for Problem 0972: Equal Rational Numbers
// Java reference: src/test/java/g0901_1000/s0972_equal_rational_numbers/SolutionTest.java

use leetcode_in_rust::s0972::equal_rational_numbers::Solution;

#[test]
fn test_is_rational_equal() {
    let result = Solution::is_rational_equal("0.(52)".to_string(), "0.5(25)".to_string());
    assert_eq!(result, true);
}

#[test]
fn test_is_rational_equal2() {
    let result = Solution::is_rational_equal("0.1666(6)".to_string(), "0.166(66)".to_string());
    assert_eq!(result, true);
}

#[test]
fn test_is_rational_equal3() {
    let result = Solution::is_rational_equal("0.9(9)".to_string(), "1.".to_string());
    assert_eq!(result, true);
}

#[test]
fn test_is_rational_equal4() {
    let result = Solution::is_rational_equal("0.08(666)".to_string(), "0.08(67)".to_string());
    assert_eq!(result, true);
}

#[test]
fn test_is_rational_equal5() {
    let result = Solution::is_rational_equal("0.(9)".to_string(), "1.".to_string());
    assert_eq!(result, true);
}

#[test]
fn test_is_rational_equal6() {
    let result = Solution::is_rational_equal("0.(99)".to_string(), "1.".to_string());
    assert_eq!(result, true);
}

#[test]
fn test_is_rational_equal7() {
    let result = Solution::is_rational_equal("0.999(99)".to_string(), "1.".to_string());
    assert_eq!(result, true);
}

#[test]
fn test_is_rational_equal8() {
    let result = Solution::is_rational_equal("0.9(59)".to_string(), "0.9(60)".to_string());
    assert_eq!(result, true);
}

#[test]
fn test_is_rational_equal9() {
    let result = Solution::is_rational_equal("0.(1)".to_string(), "0.(2)".to_string());
    assert_eq!(result, false);
}
