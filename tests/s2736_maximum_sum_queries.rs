// Tests for Problem 2736: Maximum Sum Queries
// Java reference: src/test/java/g2701_2800/s2736_maximum_sum_queries/SolutionTest.java

use leetcode_in_rust::s2736::maximum_sum_queries::Solution;

#[test]
fn test_maximum_sum_queries() {
    assert_eq!(
        Solution::maximum_sum_queries(
            vec![4, 3, 1, 2],
            vec![2, 4, 9, 5],
            vec![vec![4, 1], vec![1, 3], vec![2, 5]]
        ),
        vec![6, 10, 7]
    );
}

#[test]
fn test_maximum_sum_queries2() {
    assert_eq!(
        Solution::maximum_sum_queries(
            vec![3, 2, 5],
            vec![2, 3, 4],
            vec![vec![4, 4], vec![3, 2], vec![1, 1]]
        ),
        vec![9, 9, 9]
    );
}

#[test]
fn test_maximum_sum_queries3() {
    assert_eq!(
        Solution::maximum_sum_queries(vec![2, 1], vec![2, 3], vec![vec![3, 3]]),
        vec![-1]
    );
}
