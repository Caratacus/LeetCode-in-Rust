// Tests for Problem 2706: Buy Two Chocolates
// Java reference: src/test/java/g2701_2800/s2706_buy_two_chocolates/SolutionTest.java

use leetcode_in_rust::s2706::buy_two_chocolates::Solution;

#[test]
fn test_buy_choco() {
    assert_eq!(Solution::buy_choco(vec![1, 2, 2], 3), 0);
}

#[test]
fn test_buy_choco2() {
    assert_eq!(Solution::buy_choco(vec![3, 2, 3], 3), 3);
}
