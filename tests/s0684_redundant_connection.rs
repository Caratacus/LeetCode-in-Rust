// Tests for Problem 0684: Redundant Connection
// Java reference: src/test/java/g0601_0700/s0684_redundant_connection/SolutionTest.java

use leetcode_in_rust::s0684::redundant_connection::Solution;

#[test]
fn test_find_redundant_connection() {
    assert_eq!(
        Solution::find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![2, 3]]),
        vec![2, 3]
    );
}

#[test]
fn test_find_redundant_connection2() {
    assert_eq!(
        Solution::find_redundant_connection(vec![
            vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 4], vec![1, 5]
        ]),
        vec![1, 4]
    );
}
