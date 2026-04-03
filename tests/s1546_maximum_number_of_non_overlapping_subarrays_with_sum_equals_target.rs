// Tests for Problem 1546: Maximum Number of Non-Overlapping Subarrays With Sum Equals Target
// Java reference: src/test/java/g1501_1600/s1546_maximum_number_of_non_overlapping_subarrays_with_sum_equals_target/SolutionTest.java

use leetcode_in_rust::s1546::maximum_number_of_non_overlapping_subarrays_with_sum_equals_target::Solution;

#[test]
fn test_max_non_overlapping() {
    assert_eq!(Solution::max_non_overlapping(vec![1, 1, 1, 1, 1], 2), 2);
}

#[test]
fn test_max_non_overlapping2() {
    assert_eq!(Solution::max_non_overlapping(vec![-1, 3, 5, 1, 4, 2, -9], 6), 2);
}
