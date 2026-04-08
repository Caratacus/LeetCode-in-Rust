// Tests for Problem 3277: Maximum XOR Score Subarray Queries
// Java reference: src/test/java/g3201_3300/s3277_maximum_xor_score_subarray_queries/SolutionTest.java

use leetcode_in_rust::s3277::maximum_xor_score_subarray_queries::Solution;

#[test]
fn test_maximum_subarray_xor() {
    assert_eq!(
        Solution::maximum_subarray_xor(
            vec![2, 8, 4, 32, 16, 1],
            vec![vec![0, 2], vec![1, 4], vec![0, 5]]
        ),
        vec![12, 60, 60]
    );
}

#[test]
fn test_maximum_subarray_xor2() {
    assert_eq!(
        Solution::maximum_subarray_xor(
            vec![0, 7, 3, 2, 8, 5, 1],
            vec![vec![0, 3], vec![1, 5], vec![2, 4], vec![2, 6], vec![5, 6]]
        ),
        vec![7, 14, 11, 14, 5]
    );
}
