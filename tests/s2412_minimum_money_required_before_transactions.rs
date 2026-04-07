// Tests for Problem 2412: Minimum Money Required Before Transactions
// Java reference: src/test/java/g2401_2500/s2412_minimum_money_required_before_transactions/SolutionTest.java

use leetcode_in_rust::s2412::minimum_money_required_before_transactions::Solution;

#[test]
fn test_minimum_money() {
    assert_eq!(
        Solution::minimum_money(vec![vec![2, 1], vec![5, 0], vec![4, 2]]),
        10i64
    );
}

#[test]
fn test_minimum_money2() {
    assert_eq!(
        Solution::minimum_money(vec![vec![3, 0], vec![0, 3]]),
        3i64
    );
}
