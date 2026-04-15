// Tests for Problem 3413: Maximum Coins from K Consecutive Bags
// Java reference: src/test/java/g3401_3500/s3413_maximum_coins_from_k_consecutive_bags/SolutionTest.java

use leetcode_in_rust::s3413::maximum_coins_from_k_consecutive_bags::Solution;

#[test]
fn test_maximum_coins() {
    assert_eq!(Solution::maximum_coins(vec![vec![8, 10, 1], vec![1, 3, 2], vec![5, 6, 4]], 4), 10i64);
}

#[test]
fn test_maximum_coins2() {
    assert_eq!(Solution::maximum_coins(vec![vec![1, 10, 3]], 2), 6i64);
}
