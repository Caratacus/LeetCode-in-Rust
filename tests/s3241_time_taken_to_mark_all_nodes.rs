// Tests for Problem 3241: Time Taken to Mark All Nodes
// Java reference: src/test/java/g3201_3300/s3241_time_taken_to_mark_all_nodes/SolutionTest.java

use leetcode_in_rust::s3241::time_taken_to_mark_all_nodes::Solution;

#[test]
fn test_time_taken() {
    assert_eq!(Solution::time_taken(vec![vec![0, 1], vec![0, 2]]), vec![2, 4, 3]);
}

#[test]
fn test_time_taken2() {
    assert_eq!(Solution::time_taken(vec![vec![0, 1]]), vec![1, 2]);
}

#[test]
fn test_time_taken3() {
    assert_eq!(
        Solution::time_taken(vec![vec![2, 4], vec![0, 1], vec![2, 3], vec![0, 2]]),
        vec![4, 6, 3, 5, 5]
    );
}
