// Tests for Problem 3112: Minimum Time to Visit Disappearing Nodes
// Java reference: src/test/java/g3101_3200/s3112_minimum_time_to_visit_disappearing_nodes/SolutionTest.java

use leetcode_in_rust::s3112::minimum_time_to_visit_disappearing_nodes::Solution;

#[test]
fn test_minimum_time() {
    assert_eq!(
        Solution::minimum_time(
            3,
            vec![vec![0, 1, 2], vec![1, 2, 1], vec![0, 2, 4]],
            vec![1, 1, 5]
        ),
        vec![0, -1, 4]
    );
}

#[test]
fn test_minimum_time2() {
    assert_eq!(
        Solution::minimum_time(
            3,
            vec![vec![0, 1, 2], vec![1, 2, 1], vec![0, 2, 4]],
            vec![1, 3, 5]
        ),
        vec![0, 2, 3]
    );
}

#[test]
fn test_minimum_time3() {
    assert_eq!(
        Solution::minimum_time(2, vec![vec![0, 1, 1]], vec![1, 1]),
        vec![0, -1]
    );
}
