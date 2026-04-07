// Tests for Problem 2806: Account Balance After Rounded Purchase
// Java reference: src/test/java/g2701_2800/s2806_account_balance_after_rounded_purchase/SolutionTest.java

use leetcode_in_rust::s2806::account_balance_after_rounded_purchase::Solution;

#[test]
fn test_account_balance_after_purchase() {
    assert_eq!(Solution::account_balance_after_purchase(9), 90);
}

#[test]
fn test_account_balance_after_purchase2() {
    assert_eq!(Solution::account_balance_after_purchase(15), 80);
}
