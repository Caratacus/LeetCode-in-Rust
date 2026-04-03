// Tests for Problem 1672: Richest Customer Wealth
// Java reference: src/test/java/g1601_1700/s1672_richest_customer_wealth/SolutionTest.java

use leetcode_in_rust::s1672::richest_customer_wealth::Solution;

#[test]
fn test_maximum_wealth() {
    assert_eq!(
        Solution::maximum_wealth(vec![vec![1, 2, 3], vec![3, 2, 1]]),
        6
    );
}

#[test]
fn test_maximum_wealth2() {
    assert_eq!(
        Solution::maximum_wealth(vec![vec![1, 5], vec![7, 3], vec![3, 5]]),
        10
    );
}
