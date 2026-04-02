// Tests for Problem 0123: Best Time to Buy and Sell Stock III
// Java reference: src/test/java/g0121_0200/s0123_best_time_to_buy_and_sell_stock_iii/SolutionTest.java

use leetcode_in_rust::s0123::best_time_to_buy_and_sell_stock_iii::Solution;

#[test]
fn test_max_profit() {
    assert_eq!(Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
}

#[test]
fn test_max_profit2() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
}

#[test]
fn test_max_profit3() {
    assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
}
