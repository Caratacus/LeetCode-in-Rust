// Tests for Problem 1529: Bulb Switcher IV
// Java reference: src/test/java/g1501_1600/s1529_bulb_switcher_iv/SolutionTest.java

use leetcode_in_rust::s1529::bulb_switcher_iv::Solution;

#[test]
fn test_min_flips() {
    assert_eq!(Solution::min_flips("10111".to_string()), 3);
}

#[test]
fn test_min_flips2() {
    assert_eq!(Solution::min_flips("101".to_string()), 3);
}

#[test]
fn test_min_flips3() {
    assert_eq!(Solution::min_flips("00000".to_string()), 0);
}
