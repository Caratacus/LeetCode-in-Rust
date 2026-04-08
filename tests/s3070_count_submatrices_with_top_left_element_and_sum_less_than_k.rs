// Tests for Problem 3070: Count Submatrices With Top Left Element and Sum Less Than k
// Java reference: src/test/java/g3001_3100/s3070_count_submatrices_with_top_left_element_and_sum_less_than_k/SolutionTest.java

use leetcode_in_rust::s3070::count_submatrices_with_top_left_element_and_sum_less_than_k::Solution;

#[test]
fn test_count_submatrices() {
    assert_eq!(
        Solution::count_submatrices(vec![vec![7, 6, 3], vec![6, 6, 1]], 18),
        4
    );
}

#[test]
fn test_count_submatrices2() {
    assert_eq!(
        Solution::count_submatrices(vec![vec![7, 2, 9], vec![1, 5, 0], vec![2, 6, 6]], 20),
        6
    );
}
