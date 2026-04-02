// Tests for Problem 0520: Detect Capital
// Java reference: src/test/java/g0501_0600/s0520_detect_capital/SolutionTest.java

use leetcode_in_rust::s0520::detect_capital::Solution;

#[test]
fn test_detect_capital_use() {
    assert_eq!(Solution::detect_capital_use("USA".to_string()), true);
}

#[test]
fn test_detect_capital_use2() {
    assert_eq!(Solution::detect_capital_use("FlaG".to_string()), false);
}
