// Tests for Problem 0326: Power of Three
// Java reference: src/test/java/g0301_0400/s0326_power_of_three/SolutionTest.java

use leetcode_in_rust::s0326::power_of_three::Solution;

#[test]
fn test_is_power_of_three() {
    assert_eq!(Solution::is_power_of_three(27), true);
}

#[test]
fn test_is_power_of_three2() {
    assert_eq!(Solution::is_power_of_three(0), false);
}

#[test]
fn test_is_power_of_three3() {
    assert_eq!(Solution::is_power_of_three(9), true);
}

#[test]
fn test_is_power_of_three4() {
    assert_eq!(Solution::is_power_of_three(45), false);
}
