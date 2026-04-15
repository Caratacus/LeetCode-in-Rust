// Tests for Problem 3562: Maximum Profit from Trading Stocks with Discounts
// Java reference: src/test/java/g3501_3600/s3562_maximum_profit_from_trading_stocks_with_discounts/SolutionTest.java
use leetcode_in_rust::s3562::maximum_profit_from_trading_stocks_with_discounts::Solution;
#[test] fn test_max_profit() { assert_eq!(Solution::max_profit(2, vec![1, 2], vec![4, 3], vec![vec![1, 2]], 3), 5); }
#[test] fn test_max_profit2() { assert_eq!(Solution::max_profit(2, vec![3, 4], vec![5, 8], vec![vec![1, 2]], 4), 4); }
#[test] fn test_max_profit3() { assert_eq!(Solution::max_profit(3, vec![4, 6, 8], vec![7, 9, 11], vec![vec![1, 2], vec![1, 3]], 10), 10); }
#[test] fn test_max_profit4() { assert_eq!(Solution::max_profit(3, vec![5, 2, 3], vec![8, 5, 6], vec![vec![1, 2], vec![1, 3]], 7), 12); }
