// Tests for Problem 1552: Magnetic Force Between Two Balls
// Java reference: src/test/java/g1501_1600/s1552_magnetic_force_between_two_balls/SolutionTest.java

use leetcode_in_rust::s1552::magnetic_force_between_two_balls::Solution;

#[test]
fn test_max_distance() {
    assert_eq!(Solution::max_distance(vec![1, 2, 3, 4, 7], 3), 3);
}

#[test]
fn test_max_distance2() {
    assert_eq!(Solution::max_distance(vec![5, 4, 3, 2, 1, 1000000000], 2), 999999999);
}
