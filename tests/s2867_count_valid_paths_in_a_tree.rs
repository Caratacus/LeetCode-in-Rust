// Tests for Problem 2867: Count Valid Paths in a Tree
// Java reference: src/test/java/g2801_2900/s2867_count_valid_paths_in_a_tree/SolutionTest.java

use leetcode_in_rust::s2867::count_valid_paths_in_a_tree::Solution;

#[test]
fn test_count_paths() {
    assert_eq!(
        Solution::count_paths(5, vec![vec![1, 2], vec![1, 3], vec![2, 4], vec![2, 5]]),
        4
    );
}

#[test]
fn test_count_paths2() {
    assert_eq!(
        Solution::count_paths(6, vec![vec![1, 2], vec![1, 3], vec![2, 4], vec![3, 5], vec![3, 6]]),
        6
    );
}
