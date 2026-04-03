// Tests for Problem 1601: Maximum Number of Achievable Transfer Requests
// Java reference: src/test/java/g1501_1600/s1601_maximum_number_of_achievable_transfer_requests/SolutionTest.java

use leetcode_in_rust::s1601::maximum_number_of_achievable_transfer_requests::Solution;

#[test]
fn test_maximum_requests() {
    assert_eq!(
        Solution::maximum_requests(
            5,
            vec![vec![0, 1], vec![1, 0], vec![0, 1], vec![1, 2], vec![2, 0], vec![3, 4]]
        ),
        5
    );
}

#[test]
fn test_maximum_requests2() {
    assert_eq!(
        Solution::maximum_requests(3, vec![vec![0, 0], vec![1, 2], vec![2, 1]]),
        3
    );
}

#[test]
fn test_maximum_requests3() {
    assert_eq!(
        Solution::maximum_requests(4, vec![vec![0, 3], vec![3, 1], vec![1, 2], vec![2, 0]]),
        4
    );
}
