// Tests for Problem 3533: Concatenated Divisibility
// Java reference: src/test/java/g3501_3600/s3533_concatenated_divisibility/SolutionTest.java

use leetcode_in_rust::s3533::concatenated_divisibility::Solution;

#[test]
fn test_concatenated_divisibility() {
    assert_eq!(Solution::concatenated_divisibility(vec![3, 12, 45], 5), vec![3, 12, 45]);
}

#[test]
fn test_concatenated_divisibility2() {
    assert_eq!(Solution::concatenated_divisibility(vec![10, 5], 10), vec![5, 10]);
}

#[test]
fn test_concatenated_divisibility3() {
    assert_eq!(Solution::concatenated_divisibility(vec![1, 2, 3], 5), vec![]);
}
