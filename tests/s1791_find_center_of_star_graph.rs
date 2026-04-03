// Tests for Problem 1791: Find Center of Star Graph
// Java reference: src/test/java/g1701_1800/s1791_find_center_of_star_graph/SolutionTest.java

use leetcode_in_rust::s1791::find_center_of_star_graph::Solution;

#[test]
fn test_find_center() {
    assert_eq!(Solution::find_center(vec![vec![1, 2], vec![2, 3], vec![4, 2]]), 2);
}

#[test]
fn test_find_center2() {
    assert_eq!(Solution::find_center(vec![vec![1, 2], vec![5, 1], vec![1, 3], vec![1, 4]]), 1);
}
