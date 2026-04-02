// Tests for Problem 0925: Long Pressed Name
// Java reference: src/test/java/g0901_1000/s0925_long_pressed_name/SolutionTest.java

use leetcode_in_rust::s0925::long_pressed_name::Solution;

#[test]
fn test_is_long_pressed_name() {
    assert_eq!(Solution::is_long_pressed_name("alex".to_string(), "aaleex".to_string()), true);
}

#[test]
fn test_is_long_pressed_name2() {
    assert_eq!(Solution::is_long_pressed_name("saeed".to_string(), "ssaaedd".to_string()), false);
}

#[test]
fn test_is_long_pressed_name3() {
    assert_eq!(Solution::is_long_pressed_name("alex".to_string(), "ale".to_string()), false);
}

#[test]
fn test_is_long_pressed_name4() {
    assert_eq!(Solution::is_long_pressed_name("alex".to_string(), "alex".to_string()), true);
}
