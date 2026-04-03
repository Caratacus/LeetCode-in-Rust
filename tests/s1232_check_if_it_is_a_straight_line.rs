// Tests for Problem 1232: Check If It Is a Straight Line
// Java reference: src/test/java/g1201_1300/s1232_check_if_it_is_a_straight_line/SolutionTest.java

use leetcode_in_rust::s1232::check_if_it_is_a_straight_line::Solution;

#[test]
fn test_check_straight_line() {
    let result = Solution::check_straight_line(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7]]);
    assert_eq!(result, true);
}

#[test]
fn test_check_straight_line2() {
    let result = Solution::check_straight_line(vec![vec![1, 1], vec![2, 2], vec![3, 4], vec![4, 5], vec![5, 6], vec![7, 7]]);
    assert_eq!(result, false);
}
