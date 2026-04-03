// Tests for Problem 1742: Maximum Number of Balls in a Box
// Java reference: src/test/java/g1701_1800/s1742_maximum_number_of_balls_in_a_box/SolutionTest.java

use leetcode_in_rust::s1742::maximum_number_of_balls_in_a_box::Solution;

#[test]
fn test_count_balls() {
    assert_eq!(Solution::count_balls(1, 10), 2);
}

#[test]
fn test_count_balls2() {
    assert_eq!(Solution::count_balls(5, 15), 2);
}

#[test]
fn test_count_balls3() {
    assert_eq!(Solution::count_balls(19, 28), 2);
}
