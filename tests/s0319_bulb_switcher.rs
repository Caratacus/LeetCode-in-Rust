// Tests for Problem 0319: Bulb Switcher
// Java reference: src/test/java/g0301_0400/s0319_bulb_switcher/SolutionTest.java

use leetcode_in_rust::s0319::bulb_switcher::Solution;

#[test]
fn test_bulb_switch() {
    assert_eq!(Solution::bulb_switch(3), 1);
}

#[test]
fn test_bulb_switch2() {
    assert_eq!(Solution::bulb_switch(0), 0);
}

#[test]
fn test_bulb_switch3() {
    assert_eq!(Solution::bulb_switch(1), 1);
}

#[test]
fn test_bulb_switch4() {
    assert_eq!(Solution::bulb_switch(4), 2);
}
