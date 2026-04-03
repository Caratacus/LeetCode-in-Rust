// Tests for Problem 1589: Maximum Sum Obtained of Any Permutation
// Java reference: src/test/java/g1501_1600/s1589_maximum_sum_obtained_of_any_permutation/SolutionTest.java

use leetcode_in_rust::s1589::maximum_sum_obtained_of_any_permutation::Solution;

#[test]
fn test_max_sum_range_query() {
    assert_eq!(
        Solution::max_sum_range_query(vec![1, 2, 3, 4, 5], vec![vec![1, 3], vec![0, 1]]),
        19
    );
}

#[test]
fn test_max_sum_range_query2() {
    assert_eq!(
        Solution::max_sum_range_query(vec![1, 2, 3, 4, 5, 6], vec![vec![0, 1]]),
        11
    );
}
