// Tests for Problem 0657: Robot Return to Origin
// Java reference: src/test/java/g0601_0700/s0657_robot_return_to_origin/SolutionTest.java

use leetcode_in_rust::s0657::robot_return_to_origin::Solution;

#[test]
fn test_judge_circle() {
    assert_eq!(Solution::judge_circle("UD".to_string()), true);
}

#[test]
fn test_judge_circle2() {
    assert_eq!(Solution::judge_circle("LL".to_string()), false);
}
