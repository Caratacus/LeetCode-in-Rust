// Tests for Problem 2556: Disconnect Path in a Binary Matrix by at Most One Flip
// Java reference: src/test/java/g2501_2600/s2556_disconnect_path_in_a_binary_matrix_by_at_most_one_flip/SolutionTest.java

use leetcode_in_rust::s2556::disconnect_path_in_a_binary_matrix_by_at_most_one_flip::Solution;

#[test]
fn test_is_possible_to_cut_path() {
    assert_eq!(
        Solution::is_possible_to_cut_path(vec![vec![1, 1, 1], vec![1, 0, 0], vec![1, 1, 1]]),
        true
    );
}

#[test]
fn test_is_possible_to_cut_path2() {
    assert_eq!(
        Solution::is_possible_to_cut_path(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
        false
    );
}
