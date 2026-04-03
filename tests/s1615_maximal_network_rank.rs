// Tests for Problem 1615: Maximal Network Rank
// Java reference: src/test/java/g1601_1700/s1615_maximal_network_rank/SolutionTest.java

use leetcode_in_rust::s1615::maximal_network_rank::Solution;

#[test]
fn test_maximal_network_rank() {
    assert_eq!(
        Solution::maximal_network_rank(4, vec![vec![2, 1], vec![0, 3], vec![1, 2], vec![1, 3]]),
        4
    );
}

#[test]
fn test_maximal_network_rank2() {
    assert_eq!(
        Solution::maximal_network_rank(
            8,
            vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![2, 4], vec![5, 6], vec![5, 7]]
        ),
        5
    );
}

#[test]
fn test_maximal_network_rank3() {
    assert_eq!(
        Solution::maximal_network_rank(
            5,
            vec![vec![0, 1], vec![0, 3], vec![1, 2], vec![1, 3], vec![2, 3], vec![2, 4]]
        ),
        5
    );
}
