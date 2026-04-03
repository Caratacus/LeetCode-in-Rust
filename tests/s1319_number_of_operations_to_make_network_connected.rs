// Tests for Problem 1319: Number of Operations to Make Network Connected
// Java reference: src/test/java/g1301_1400/s1319_number_of_operations_to_make_network_connected/SolutionTest.java

use leetcode_in_rust::s1319::number_of_operations_to_make_network_connected::Solution;

#[test]
fn test_make_connected() {
    assert_eq!(Solution::make_connected(4, vec![vec![0, 1], vec![0, 2], vec![1, 2]]), 1);
}

#[test]
fn test_make_connected2() {
    assert_eq!(
        Solution::make_connected(6, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2], vec![1, 3]]),
        2
    );
}

#[test]
fn test_make_connected3() {
    assert_eq!(
        Solution::make_connected(6, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2]]),
        -1
    );
}
