// Tests for Problem 0860: Lemonade Change
// Java reference: src/test/java/g0801_0900/s0860_lemonade_change/SolutionTest.java

use leetcode_in_rust::s0860::lemonade_change::Solution;

#[test]
fn test_lemonade_change() {
    assert_eq!(Solution::lemonade_change(vec![5, 5, 5, 10, 20]), true);
}

#[test]
fn test_lemonade_change2() {
    assert_eq!(Solution::lemonade_change(vec![5, 5, 10, 10, 20]), false);
}
