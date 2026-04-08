// Tests for Problem 3036: Number of Subarrays That Match a Pattern II
// Java reference: src/test/java/g3001_3100/s3036_number_of_subarrays_that_match_a_pattern_ii/SolutionTest.java

use leetcode_in_rust::s3036::number_of_subarrays_that_match_a_pattern_ii::Solution;

#[test]
fn test_count_matching_subarrays() {
    assert_eq!(
        Solution::count_matching_subarrays(vec![1, 2, 3, 4, 5, 6], vec![1, 1]),
        4
    );
}

#[test]
fn test_count_matching_subarrays2() {
    assert_eq!(
        Solution::count_matching_subarrays(vec![1, 4, 4, 1, 3, 5, 5, 3], vec![1, 0, -1]),
        2
    );
}
