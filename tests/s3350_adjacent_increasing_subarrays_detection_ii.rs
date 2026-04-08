// Tests for Problem 3350: Adjacent Increasing Subarrays Detection II
// Java reference: src/test/java/g3301_3400/s3350_adjacent_increasing_subarrays_detection_ii/SolutionTest.java

use leetcode_in_rust::s3350::adjacent_increasing_subarrays_detection_ii::Solution;

#[test]
fn test_max_increasing_subarrays() {
    assert_eq!(
        Solution::max_increasing_subarrays(vec![2, 5, 7, 8, 9, 2, 3, 4, 3, 1]),
        3
    );
}

#[test]
fn test_max_increasing_subarrays2() {
    assert_eq!(
        Solution::max_increasing_subarrays(vec![1, 2, 3, 4, 4, 4, 4, 5, 6, 7]),
        2
    );
}
