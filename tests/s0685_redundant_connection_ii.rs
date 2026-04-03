// Tests for Problem 0685: Redundant Connection II
// Java reference: src/test/java/g0601_0700/s0685_redundant_connection_ii/SolutionTest.java

use leetcode_in_rust::s0685::redundant_connection_ii::Solution;

#[test]
fn test_find_redundant_directed_connection() {
    assert_eq!(
        Solution::find_redundant_directed_connection(vec![vec![1, 2], vec![1, 3], vec![2, 3]]),
        vec![2, 3]
    );
}

#[test]
fn test_find_redundant_directed_connection2() {
    assert_eq!(
        Solution::find_redundant_directed_connection(vec![
            vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 1], vec![1, 5]
        ]),
        vec![4, 1]
    );
}
