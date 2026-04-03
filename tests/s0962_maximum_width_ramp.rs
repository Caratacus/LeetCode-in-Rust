// Tests for Problem 0962: Maximum Width Ramp
// Java reference: src/test/java/g0901_1000/s0962_maximum_width_ramp/SolutionTest.java

use leetcode_in_rust::s0962::maximum_width_ramp::Solution;

#[test]
fn test_max_width_ramp() {
    let result = Solution::max_width_ramp(vec![6, 0, 8, 2, 1, 5]);
    assert_eq!(result, 4);
}

#[test]
fn test_max_width_ramp2() {
    let result = Solution::max_width_ramp(vec![9, 8, 1, 0, 1, 9, 4, 0, 4, 1]);
    assert_eq!(result, 7);
}
