// Tests for Problem 3395: Subsequences With a Unique Middle Mode I
// Java reference: src/test/java/g3301_3400/s3395_subsequences_with_a_unique_middle_mode_i/SolutionTest.java

use leetcode_in_rust::s3395::subsequences_with_a_unique_middle_mode_i::Solution;

#[test]
fn test_subsequences_with_middle_mode() {
    assert_eq!(Solution::subsequences_with_middle_mode(vec![1, 1, 1, 1, 1, 1]), 6);
}

#[test]
fn test_subsequences_with_middle_mode2() {
    assert_eq!(Solution::subsequences_with_middle_mode(vec![1, 2, 2, 3, 3, 4]), 4);
}

#[test]
fn test_subsequences_with_middle_mode3() {
    assert_eq!(
        Solution::subsequences_with_middle_mode(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]),
        0
    );
}
