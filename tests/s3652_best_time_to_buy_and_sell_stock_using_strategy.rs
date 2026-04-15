// Tests for Problem 3652: Best Time to Buy and Sell Stock Using Strategy
// Java reference: src/test/java/g3601_3700/s3652_best_time_to_buy_and_sell_stock_using_strategy/SolutionTest.java
use leetcode_in_rust::s3652::best_time_to_buy_and_sell_stock_using_strategy::Solution;
#[test]
fn test_max_profit() { assert_eq!(Solution::max_profit(vec![4, 2, 8], vec![-1, 0, 1], 2), 10i64); }
#[test]
fn test_max_profit2() { assert_eq!(Solution::max_profit(vec![5, 4, 3], vec![1, 1, 0], 2), 9i64); }
