// Tests for Problem 0122: Best Time to Buy and Sell Stock II
// Java reference: src/test/java/g0121_0200/s0122_best_time_to_buy_and_sell_stock_ii/SolutionTest.java

use leetcode_in_rust::s0122::best_time_to_buy_and_sell_stock_ii::Solution;

#[test]
fn test_max_profit() {
    assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
}

#[test]
fn test_max_profit2() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
}

#[test]
fn test_max_profit3() {
    assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
}
