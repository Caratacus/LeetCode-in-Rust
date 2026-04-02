// Tests for Problem 0188: Best Time to Buy and Sell Stock IV
// Java reference: src/test/java/g0181_0200/s0188_best_time_to_buy_and_sell_stock_iv/SolutionTest.java

use leetcode_in_rust::s0188::best_time_to_buy_and_sell_stock_iv::Solution;

#[test]
fn test_max_profit() {
    assert_eq!(Solution::max_profit(2, vec![2, 4, 1]), 2);
}

#[test]
fn test_max_profit2() {
    assert_eq!(Solution::max_profit(2, vec![3, 2, 6, 5, 0, 3]), 7);
}
