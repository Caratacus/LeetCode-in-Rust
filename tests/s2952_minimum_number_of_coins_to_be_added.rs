// Tests for Problem 2952: Minimum Number of Coins to be Added
// Java reference: src/test/java/g2901_3000/s2952_minimum_number_of_coins_to_be_added/SolutionTest.java

use leetcode_in_rust::s2952::minimum_number_of_coins_to_be_added::Solution;

#[test]
fn test_minimum_added_coins() {
    assert_eq!(Solution::minimum_added_coins(vec![1, 4, 10], 19), 2);
}

#[test]
fn test_minimum_added_coins2() {
    assert_eq!(Solution::minimum_added_coins(vec![1, 4, 10, 5, 7, 19], 19), 1);
}

#[test]
fn test_minimum_added_coins3() {
    assert_eq!(Solution::minimum_added_coins(vec![1, 1, 1], 20), 3);
}
