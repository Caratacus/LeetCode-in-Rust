// Tests for Problem 2104: Sum of Subarray Ranges
// Java reference: src/test/java/g2101_2200/s2104_sum_of_subarray_ranges/SolutionTest.java

use leetcode_in_rust::s2104::sum_of_subarray_ranges::Solution;

#[test]
fn test_sub_array_ranges() {
    assert_eq!(Solution::sub_array_ranges(vec![1, 2, 3]), 4);
}

#[test]
fn test_sub_array_ranges2() {
    assert_eq!(Solution::sub_array_ranges(vec![1, 3, 3]), 4);
}

#[test]
fn test_sub_array_ranges3() {
    assert_eq!(Solution::sub_array_ranges(vec![4, -2, -3, 4, 1]), 59);
}
