// Tests for Problem 1561: Maximum Number of Coins You Can Get
// Java reference: src/test/java/g1501_1600/s1561_maximum_number_of_coins_you_can_get/SolutionTest.java

use leetcode_in_rust::s1561::maximum_number_of_coins_you_can_get::Solution;

#[test]
fn test_max_coins() {
    assert_eq!(Solution::max_coins(vec![2, 4, 1, 2, 7, 8]), 9);
}

#[test]
fn test_max_coins2() {
    assert_eq!(Solution::max_coins(vec![2, 4, 5]), 4);
}

#[test]
fn test_max_coins3() {
    assert_eq!(Solution::max_coins(vec![9, 8, 7, 6, 5, 1, 2, 3, 4]), 18);
}
