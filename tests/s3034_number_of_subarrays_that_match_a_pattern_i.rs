// Tests for Problem 3034: Number of Subarrays That Match a Pattern I
// Java reference: src/test/java/g3001_3100/s3034_number_of_subarrays_that_match_a_pattern_i/SolutionTest.java

use leetcode_in_rust::s3034::number_of_subarrays_that_match_a_pattern_i::Solution;

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
