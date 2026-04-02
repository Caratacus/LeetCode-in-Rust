// Tests for Problem 0207: Course Schedule
// Java reference: src/test/java/g0201_0300/s0207_course_schedule/SolutionTest.java

use leetcode_in_rust::s0207::course_schedule::Solution;

#[test]
fn test_can_finish() {
    assert_eq!(Solution::can_finish(2, vec![vec![1, 0]]), true);
}

#[test]
fn test_can_finish2() {
    assert_eq!(Solution::can_finish(2, vec![vec![1, 0], vec![0, 1]]), false);
}
