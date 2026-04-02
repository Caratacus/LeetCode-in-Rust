// Tests for Problem 0335: Self Crossing
// Java reference: src/test/java/g0301_0400/s0335_self_crossing/SolutionTest.java

use leetcode_in_rust::s0335::self_crossing::Solution;

#[test]
fn test_is_self_crossing() {
    assert_eq!(Solution::is_self_crossing(vec![2, 1, 1, 2]), true);
}

#[test]
fn test_is_self_crossing2() {
    assert_eq!(Solution::is_self_crossing(vec![1, 2, 3, 4]), false);
}

#[test]
fn test_is_self_crossing3() {
    assert_eq!(Solution::is_self_crossing(vec![1, 1, 2, 1, 1]), true);
}
