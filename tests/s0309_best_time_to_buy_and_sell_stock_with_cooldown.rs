// Tests for Problem 0309: Best Time to Buy and Sell Stock with Cooldown
// Java reference: src/test/java/g0301_0400/s0309_best_time_to_buy_and_sell_stock_with_cooldown/SolutionTest.java

use leetcode_in_rust::s0309::best_time_to_buy_and_sell_stock_with_cooldown::Solution;

#[test]
fn test_max_profit() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 2]), 3);
}

#[test]
fn test_max_profit2() {
    assert_eq!(Solution::max_profit(vec![1]), 0);
}
