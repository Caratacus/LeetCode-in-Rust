// Tests for Problem 0231: Power of Two
// Java reference: src/test/java/g0201_0300/s0231_power_of_two/SolutionTest.java

use leetcode_in_rust::s0231::power_of_two::Solution;

#[test]
fn test_is_power_of_two() {
    assert_eq!(Solution::is_power_of_two(1), true);
}

#[test]
fn test_is_power_of_two2() {
    assert_eq!(Solution::is_power_of_two(16), true);
}

#[test]
fn test_is_power_of_two3() {
    assert_eq!(Solution::is_power_of_two(3), false);
}
