// Tests for Problem 1582: Special Positions in a Binary Matrix
// Java reference: src/test/java/g1501_1600/s1582_special_positions_in_a_binary_matrix/SolutionTest.java

use leetcode_in_rust::s1582::special_positions_in_a_binary_matrix::Solution;

#[test]
fn test_num_special() {
    assert_eq!(
        Solution::num_special(vec![vec![1, 0, 0], vec![0, 0, 1], vec![1, 0, 0]]),
        1
    );
}

#[test]
fn test_num_special2() {
    assert_eq!(
        Solution::num_special(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]),
        3
    );
}
