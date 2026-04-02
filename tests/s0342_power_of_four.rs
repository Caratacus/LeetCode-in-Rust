// Tests for Problem 0342: Power of Four
// Java reference: src/test/java/g0301_0400/s0342_power_of_four/SolutionTest.java

use leetcode_in_rust::s0342::power_of_four::Solution;

#[test]
fn test_is_power_of_four() {
    assert_eq!(Solution::is_power_of_four(16), true);
}

#[test]
fn test_is_power_of_four2() {
    assert_eq!(Solution::is_power_of_four(5), false);
}

#[test]
fn test_is_power_of_four3() {
    assert_eq!(Solution::is_power_of_four(1), true);
}
