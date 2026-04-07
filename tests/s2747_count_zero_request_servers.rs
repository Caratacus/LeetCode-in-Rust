// Tests for Problem 2747: Count Zero Request Servers
// Java reference: src/test/java/g2701_2800/s2747_count_zero_request_servers/SolutionTest.java

use leetcode_in_rust::s2747::count_zero_request_servers::Solution;

#[test]
fn test_count_servers() {
    assert_eq!(
        Solution::count_servers(
            3,
            vec![vec![1, 3], vec![2, 6], vec![1, 5]],
            5,
            vec![10, 11]
        ),
        vec![1, 2]
    );
}

#[test]
fn test_count_servers2() {
    assert_eq!(
        Solution::count_servers(
            3,
            vec![vec![2, 4], vec![2, 1], vec![1, 2], vec![3, 1], vec![2, 3], vec![1, 1]],
            2,
            vec![3, 4, 5, 6]
        ),
        vec![1, 1, 0, 1]
    );
}
