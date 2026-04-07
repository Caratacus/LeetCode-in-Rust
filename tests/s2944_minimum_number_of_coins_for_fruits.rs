// Tests for Problem 2944: Minimum Number of Coins for Fruits
// Java reference: src/test/java/g2901_3000/s2944_minimum_number_of_coins_for_fruits/SolutionTest.java

use leetcode_in_rust::s2944::minimum_number_of_coins_for_fruits::Solution;

#[test]
fn test_minimum_coins() {
    assert_eq!(Solution::minimum_coins(vec![3, 1, 2]), 4);
}

#[test]
fn test_minimum_coins2() {
    assert_eq!(Solution::minimum_coins(vec![1, 10, 1, 1]), 2);
}
