// Tests for Problem 2536: Increment Submatrices by One
// Java reference: src/test/java/g2501_2600/s2536_increment_submatrices_by_one/SolutionTest.java

use leetcode_in_rust::s2536::increment_submatrices_by_one::Solution;

#[test]
fn test_range_add_queries() {
    assert_eq!(
        Solution::range_add_queries(3, vec![vec![1, 1, 2, 2], vec![0, 0, 1, 1]]),
        vec![vec![1, 1, 0], vec![1, 2, 1], vec![0, 1, 1]]
    );
}

#[test]
fn test_range_add_queries2() {
    assert_eq!(
        Solution::range_add_queries(2, vec![vec![0, 0, 1, 1]]),
        vec![vec![1, 1], vec![1, 1]]
    );
}
