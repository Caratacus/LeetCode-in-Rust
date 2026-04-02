// Tests for Problem 0518: Coin Change 2
// Java reference: src/test/java/g0501_0600/s0518_coin_change_2/SolutionTest.java

use leetcode_in_rust::s0518::coin_change_2::Solution;

#[test]
fn test_change() {
    assert_eq!(Solution::change(5, vec![1, 2, 5]), 4);
}

#[test]
fn test_change2() {
    assert_eq!(Solution::change(3, vec![2]), 0);
}

#[test]
fn test_change3() {
    assert_eq!(Solution::change(10, vec![10]), 1);
}
