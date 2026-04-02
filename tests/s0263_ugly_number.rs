// Tests for Problem 0263: Ugly Number
// Java reference: src/test/java/g0201_0300/s0263_ugly_number/SolutionTest.java

use leetcode_in_rust::s0263::ugly_number::Solution;

#[test]
fn test_is_ugly() {
    assert_eq!(Solution::is_ugly(6), true);
}

#[test]
fn test_is_ugly2() {
    assert_eq!(Solution::is_ugly(8), true);
}

#[test]
fn test_is_ugly3() {
    assert_eq!(Solution::is_ugly(14), false);
}

#[test]
fn test_is_ugly4() {
    assert_eq!(Solution::is_ugly(1), true);
}
