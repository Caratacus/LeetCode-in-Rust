// Tests for Problem 3349: Adjacent Increasing Subarrays Detection I
// Java reference: src/test/java/g3301_3400/s3349_adjacent_increasing_subarrays_detection_i/SolutionTest.java

use leetcode_in_rust::s3349::adjacent_increasing_subarrays_detection_i::Solution;

#[test]
fn test_has_increasing_subarrays() {
    assert_eq!(
        Solution::has_increasing_subarrays(vec![2, 5, 7, 8, 9, 2, 3, 4, 3, 1], 3),
        true
    );
}

#[test]
fn test_has_increasing_subarrays2() {
    assert_eq!(
        Solution::has_increasing_subarrays(vec![1, 2, 3, 4, 4, 4, 4, 5, 6, 7], 5),
        false
    );
}
