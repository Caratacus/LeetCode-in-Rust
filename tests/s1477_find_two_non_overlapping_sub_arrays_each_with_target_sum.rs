// Tests for Problem 1477: Find Two Non-overlapping Sub-arrays Each With Target Sum
// Java reference: src/test/java/g1401_1500/s1477_find_two_non_overlapping_sub_arrays_each_with_target_sum/SolutionTest.java

use leetcode_in_rust::s1477::find_two_non_overlapping_sub_arrays_each_with_target_sum::Solution;

#[test]
fn test_min_sum_of_lengths() {
    assert_eq!(Solution::min_sum_of_lengths(vec![3, 2, 2, 4, 3], 3), 2);
}

#[test]
fn test_min_sum_of_lengths2() {
    assert_eq!(Solution::min_sum_of_lengths(vec![7, 3, 4, 7], 7), 2);
}

#[test]
fn test_min_sum_of_lengths3() {
    assert_eq!(Solution::min_sum_of_lengths(vec![4, 3, 2, 6, 2, 3, 4], 6), -1);
}
