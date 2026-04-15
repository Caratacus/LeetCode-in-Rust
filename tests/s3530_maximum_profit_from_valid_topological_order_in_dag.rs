// Tests for Problem 3530: Maximum Profit from Valid Topological Order in DAG
// Java reference: src/test/java/g3501_3600/s3530_maximum_profit_from_valid_topological_order_in_dag/SolutionTest.java

use leetcode_in_rust::s3530::maximum_profit_from_valid_topological_order_in_dag::Solution;

#[test]
fn test_max_profit() {
    assert_eq!(Solution::max_profit(2, vec![vec![0, 1]], vec![2, 3]), 8);
}

#[test]
fn test_max_profit2() {
    assert_eq!(Solution::max_profit(3, vec![vec![0, 1], vec![0, 2]], vec![1, 6, 3]), 25);
}
