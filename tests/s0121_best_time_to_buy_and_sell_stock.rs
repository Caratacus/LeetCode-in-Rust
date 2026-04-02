// Tests for Problem 0121: Best Time to Buy and Sell Stock
// Java reference: src/test/java/g0121_0200/s0121_best_time_to_buy_and_sell_stock/SolutionTest.java

use leetcode_in_rust::s0121::best_time_to_buy_and_sell_stock::Solution;

#[test]
fn test_max_profit() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
}

#[test]
fn test_max_profit2() {
    assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
}

#[test]
fn test_max_profit3() {
    assert_eq!(Solution::max_profit(vec![1]), 0);
}
