// Tests for Problem 0740: Delete and Earn
// Java reference: src/test/java/g0701_0800/s0740_delete_and_earn/SolutionTest.java

use leetcode_in_rust::s0740::delete_and_earn::Solution;

#[test]
fn test_delete_and_earn() {
    assert_eq!(Solution::delete_and_earn(vec![3, 4, 3]), 6);
}

#[test]
fn test_delete_and_earn2() {
    assert_eq!(Solution::delete_and_earn(vec![2, 2, 3, 3, 3, 4]), 9);
}
