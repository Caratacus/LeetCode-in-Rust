// Tests for Problem 0322: Coin Change
// Java reference: src/test/java/g0301_0400/s0322_coin_change/SolutionTest.java

use leetcode_in_rust::s0322::coin_change::Solution;

#[test]
fn test_coin_change() {
    assert_eq!(Solution::coin_change(vec![1, 2, 5], 11), 3);
}

#[test]
fn test_coin_change2() {
    assert_eq!(Solution::coin_change(vec![2], 3), -1);
}

#[test]
fn test_coin_change3() {
    assert_eq!(Solution::coin_change(vec![1], 0), 0);
}
