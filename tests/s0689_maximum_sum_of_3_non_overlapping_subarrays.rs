// Tests for Problem 0689: Maximum Sum of 3 Non-Overlapping Subarrays
// Java reference: src/test/java/g0601_0700/s0689_maximum_sum_of_3_non_overlapping_subarrays/SolutionTest.java

use leetcode_in_rust::s0689::maximum_sum_of_3_non_overlapping_subarrays::Solution;

#[test]
fn test_max_sum_of_three_subarrays() {
    assert_eq!(
        Solution::max_sum_of_three_subarrays(vec![1, 2, 1, 2, 6, 7, 5, 1], 2),
        vec![0, 3, 5]
    );
}

#[test]
fn test_max_sum_of_three_subarrays2() {
    assert_eq!(
        Solution::max_sum_of_three_subarrays(vec![1, 2, 1, 2, 1, 2, 1, 2, 1], 2),
        vec![0, 2, 4]
    );
}
