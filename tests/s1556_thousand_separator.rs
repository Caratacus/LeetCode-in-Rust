// Tests for Problem 1556: Thousand Separator
// Java reference: src/test/java/g1501_1600/s1556_thousand_separator/SolutionTest.java

use leetcode_in_rust::s1556::thousand_separator::Solution;

#[test]
fn test_thousand_separator() {
    assert_eq!(Solution::thousand_separator(987), "987");
}

#[test]
fn test_thousand_separator2() {
    assert_eq!(Solution::thousand_separator(1234), "1.234");
}
