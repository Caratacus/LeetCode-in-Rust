// Tests for Problem 3178: Find the Child Who Has the Ball After K Seconds
// Java reference: src/test/java/g3101_3200/s3178_find_the_child_who_has_the_ball_after_k_seconds/SolutionTest.java

use leetcode_in_rust::s3178::find_the_child_who_has_the_ball_after_k_seconds::Solution;

#[test]
fn test_number_of_child() {
    assert_eq!(Solution::number_of_child(3, 5), 1);
}
#[test]
fn test_number_of_child2() {
    assert_eq!(Solution::number_of_child(5, 6), 2);
}
#[test]
fn test_number_of_child3() {
    assert_eq!(Solution::number_of_child(4, 2), 2);
}