// Tests for Problem 2569: Handling Sum Queries After Update
// Java reference: src/test/java/g2501_2600/s2569_handling_sum_queries_after_update/SolutionTest.java

use leetcode_in_rust::s2569::handling_sum_queries_after_update::Solution;

#[test]
fn test_handle_query() {
    assert_eq!(
        Solution::handle_query(
            vec![1, 0, 1],
            vec![0, 0, 0],
            vec![vec![1, 1, 1], vec![2, 1, 0], vec![3, 0, 0]]
        ),
        vec![3]
    );
}

#[test]
fn test_handle_query2() {
    assert_eq!(
        Solution::handle_query(
            vec![1],
            vec![5],
            vec![vec![2, 0, 0], vec![3, 0, 0]]
        ),
        vec![5]
    );
}
