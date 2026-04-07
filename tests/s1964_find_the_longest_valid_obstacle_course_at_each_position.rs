// Tests for Problem 1964: Find the Longest Valid Obstacle Course at Each Position
// Java reference: src/test/java/g1901_2000/s1964_find_the_longest_valid_obstacle_course_at_each_position/SolutionTest.java

use leetcode_in_rust::s1964::find_the_longest_valid_obstacle_course_at_each_position::Solution;

#[test]
fn test_longest_obstacle_course_at_each_position() {
    assert_eq!(
        Solution::longest_obstacle_course_at_each_position(vec![1, 2, 3, 2]),
        vec![1, 2, 3, 3]
    );
}

#[test]
fn test_longest_obstacle_course_at_each_position2() {
    assert_eq!(
        Solution::longest_obstacle_course_at_each_position(vec![2, 2, 1]),
        vec![1, 2, 1]
    );
}

#[test]
fn test_longest_obstacle_course_at_each_position3() {
    assert_eq!(
        Solution::longest_obstacle_course_at_each_position(vec![3, 1, 5, 6, 4, 2]),
        vec![1, 1, 2, 3, 2, 2]
    );
}
