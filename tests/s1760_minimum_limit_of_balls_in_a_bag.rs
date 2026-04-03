// Tests for Problem 1760: Minimum Limit of Balls in a Bag
// Java reference: src/test/java/g1701_1800/s1760_minimum_limit_of_balls_in_a_bag/SolutionTest.java

use leetcode_in_rust::s1760::minimum_limit_of_balls_in_a_bag::Solution;

#[test]
fn test_minimum_size() {
    assert_eq!(Solution::minimum_size(vec![9], 2), 3);
}

#[test]
fn test_minimum_size2() {
    assert_eq!(Solution::minimum_size(vec![2, 4, 8, 2], 4), 2);
}

#[test]
fn test_minimum_size3() {
    assert_eq!(Solution::minimum_size(vec![7, 17], 2), 7);
}
