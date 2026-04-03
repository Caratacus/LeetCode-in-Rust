// Tests for Problem 1344: Angle Between Hands of a Clock
// Java reference: src/test/java/g1301_1400/s1344_angle_between_hands_of_a_clock/SolutionTest.java

use leetcode_in_rust::s1344::angle_between_hands_of_a_clock::Solution;

#[test]
fn test_angle_clock() {
    assert_eq!(Solution::angle_clock(12, 30), 165.0);
}

#[test]
fn test_angle_clock2() {
    assert_eq!(Solution::angle_clock(3, 30), 75.0);
}

#[test]
fn test_angle_clock3() {
    assert_eq!(Solution::angle_clock(3, 15), 7.5);
}
