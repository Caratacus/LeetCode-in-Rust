// Tests for Problem 0764: Largest Plus Sign
// Java reference: src/test/java/g0701_0800/s0764_largest_plus_sign/SolutionTest.java

use leetcode_in_rust::s0764::largest_plus_sign::Solution;

#[test]
fn test_order_of_largest_plus_sign() {
    assert_eq!(Solution::order_of_largest_plus_sign(5, vec![vec![4, 2]]), 2);
}

#[test]
fn test_order_of_largest_plus_sign2() {
    assert_eq!(Solution::order_of_largest_plus_sign(1, vec![vec![0, 0]]), 0);
}
