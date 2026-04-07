// Tests for Problem 2076: Process Restricted Friend Requests
// Java reference: src/test/java/g2001_2100/s2076_process_restricted_friend_requests/SolutionTest.java

use leetcode_in_rust::s2076::process_restricted_friend_requests::Solution;

#[test]
fn test_friend_requests() {
    assert_eq!(
        Solution::friend_requests(3, vec![vec![0, 1]], vec![vec![0, 2], vec![2, 1]]),
        vec![true, false]
    );
}

#[test]
fn test_friend_requests2() {
    assert_eq!(
        Solution::friend_requests(3, vec![vec![0, 1]], vec![vec![1, 2], vec![0, 2]]),
        vec![true, false]
    );
}

#[test]
fn test_friend_requests3() {
    assert_eq!(
        Solution::friend_requests(
            5,
            vec![vec![0, 1], vec![1, 2], vec![2, 3]],
            vec![vec![0, 4], vec![1, 2], vec![3, 1], vec![3, 4]]
        ),
        vec![true, false, true, false]
    );
}
