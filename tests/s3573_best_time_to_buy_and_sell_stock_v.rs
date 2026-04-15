// Tests for Problem 3573: Best Time to Buy and Sell Stock V
// Java reference: src/test/java/g3501_3600/s3573_best_time_to_buy_and_sell_stock_v/SolutionTest.java
use leetcode_in_rust::s3573::best_time_to_buy_and_sell_stock_v::Solution;
#[test] fn test_maximum_profit() { assert_eq!(Solution::maximum_profit(vec![1, 7, 9, 8, 2], 2), 14i64); }
#[test] fn test_maximum_profit2() { assert_eq!(Solution::maximum_profit(vec![12, 16, 19, 19, 8, 1, 19, 13, 9], 3), 36i64); }
