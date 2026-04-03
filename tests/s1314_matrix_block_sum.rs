// Tests for Problem 1314: Matrix Block Sum
// Java reference: src/test/java/g1301_1400/s1314_matrix_block_sum/SolutionTest.java

use leetcode_in_rust::s1314::matrix_block_sum::Solution;

#[test]
fn test_matrix_block_sum() {
    assert_eq!(
        Solution::matrix_block_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 1),
        vec![vec![12, 21, 16], vec![27, 45, 33], vec![24, 39, 28]]
    );
}

#[test]
fn test_matrix_block_sum2() {
    assert_eq!(
        Solution::matrix_block_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 2),
        vec![vec![45, 45, 45], vec![45, 45, 45], vec![45, 45, 45]]
    );
}
