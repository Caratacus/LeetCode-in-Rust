// Tests for Problem 1719: Number of Ways to Reconstruct a Tree
// Java reference: src/test/java/g1701_1800/s1719_number_of_ways_to_reconstruct_a_tree/SolutionTest.java

use leetcode_in_rust::s1719::number_of_ways_to_reconstruct_a_tree::Solution;

#[test]
fn test_check_ways() {
    assert_eq!(Solution::check_ways(vec![vec![1, 2], vec![2, 3]]), 1);
}

#[test]
fn test_check_ways2() {
    assert_eq!(
        Solution::check_ways(vec![vec![1, 2], vec![2, 3], vec![1, 3]]),
        2
    );
}

#[test]
fn test_check_ways3() {
    assert_eq!(
        Solution::check_ways(vec![vec![1, 2], vec![2, 3], vec![2, 4], vec![1, 5]]),
        0
    );
}
