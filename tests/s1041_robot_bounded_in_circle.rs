// Tests for Problem 1041: Robot Bounded In Circle
// Java reference: src/test/java/g1001_1100/s1041_robot_bounded_in_circle/SolutionTest.java

use leetcode_in_rust::s1041::robot_bounded_in_circle::Solution;

#[test]
fn test_is_robot_bounded() {
    assert_eq!(Solution::is_robot_bounded("GGLLGG".to_string()), true);
}

#[test]
fn test_is_robot_bounded2() {
    assert_eq!(Solution::is_robot_bounded("GG".to_string()), false);
}

#[test]
fn test_is_robot_bounded3() {
    assert_eq!(Solution::is_robot_bounded("GL".to_string()), true);
}
