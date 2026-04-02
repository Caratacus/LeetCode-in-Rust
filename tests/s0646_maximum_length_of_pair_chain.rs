// Tests for Problem 0646: Maximum Length of Pair Chain
// Java reference: src/test/java/g0601_0700/s0646_maximum_length_of_pair_chain/SolutionTest.java

use leetcode_in_rust::s0646::maximum_length_of_pair_chain::Solution;

#[test]
fn test_find_longest_chain() {
    assert_eq!(
        Solution::find_longest_chain(vec![vec![1, 2], vec![2, 3], vec![3, 4]]),
        2
    );
}

#[test]
fn test_find_longest_chain2() {
    assert_eq!(
        Solution::find_longest_chain(vec![vec![1, 2], vec![7, 8], vec![4, 5]]),
        3
    );
}
