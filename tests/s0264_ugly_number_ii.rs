// Tests for Problem 0264: Ugly Number II
// Java reference: src/test/java/g0201_0300/s0264_ugly_number_ii/SolutionTest.java

use leetcode_in_rust::s0264::ugly_number_ii::Solution;

#[test]
fn test_nth_ugly_number() {
    assert_eq!(Solution::nth_ugly_number(10), 12);
}

#[test]
fn test_nth_ugly_number2() {
    assert_eq!(Solution::nth_ugly_number(1), 1);
}
