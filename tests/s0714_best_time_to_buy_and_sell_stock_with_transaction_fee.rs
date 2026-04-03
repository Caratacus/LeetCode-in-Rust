// Tests for Problem 0714: Best Time to Buy and Sell Stock with Transaction Fee
// Java reference: src/test/java/g0701_0800/s0714_best_time_to_buy_and_sell_stock_with_transaction_fee/SolutionTest.java

use leetcode_in_rust::s0714::best_time_to_buy_and_sell_stock_with_transaction_fee::Solution;

#[test]
fn test_max_profit() {
    assert_eq!(Solution::max_profit(vec![1, 3, 2, 8, 4, 9], 2), 8);
}

#[test]
fn test_max_profit2() {
    assert_eq!(Solution::max_profit(vec![1, 3, 7, 5, 10, 3], 3), 6);
}
