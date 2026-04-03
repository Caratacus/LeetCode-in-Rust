// Tests for Problem 1504: Count Submatrices With All Ones
// Java reference: src/test/java/g1501_1600/s1504_count_submatrices_with_all_ones/SolutionTest.java

use leetcode_in_rust::s1504::count_submatrices_with_all_ones::Solution;

#[test]
fn test_num_submat() {
    let mat = vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]];
    assert_eq!(Solution::num_submat(mat), 13);
}

#[test]
fn test_num_submat2() {
    let mat = vec![vec![0, 1, 1, 0], vec![0, 1, 1, 1], vec![1, 1, 1, 0]];
    assert_eq!(Solution::num_submat(mat), 24);
}
