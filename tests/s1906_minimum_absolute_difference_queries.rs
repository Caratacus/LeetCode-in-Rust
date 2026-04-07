// Tests for Problem 1906: Minimum Absolute Difference Queries
// Java reference: src/test/java/g1901_2000/s1906_minimum_absolute_difference_queries/SolutionTest.java

use leetcode_in_rust::s1906::minimum_absolute_difference_queries::Solution;

#[test]
fn test_min_difference() {
    assert_eq!(
        Solution::min_difference(
            vec![1, 3, 4, 8],
            vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![0, 3]]
        ),
        vec![2, 1, 4, 1]
    );
}

#[test]
fn test_min_difference2() {
    assert_eq!(
        Solution::min_difference(
            vec![4, 5, 2, 2, 7, 10],
            vec![vec![2, 3], vec![0, 2], vec![0, 5], vec![3, 5]]
        ),
        vec![-1, 1, 1, 3]
    );
}
