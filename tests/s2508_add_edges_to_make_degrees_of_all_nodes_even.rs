// Tests for Problem 2508: Add Edges to Make Degrees of All Nodes Even
// Java reference: src/test/java/g2401_2500/s2508_add_edges_to_make_degrees_of_all_nodes_even/SolutionTest.java

use leetcode_in_rust::s2508::add_edges_to_make_degrees_of_all_nodes_even::Solution;

#[test]
fn test_is_possible() {
    assert_eq!(Solution::is_possible(5, vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 2], vec![1, 4]]), true);
}

#[test]
fn test_is_possible2() {
    assert_eq!(Solution::is_possible(4, vec![vec![1, 2], vec![3, 4]]), false);
}

#[test]
fn test_is_possible3() {
    assert_eq!(Solution::is_possible(4, vec![vec![1, 2], vec![1, 3], vec![1, 4]]), true);
}

#[test]
fn test_is_possible4() {
    assert_eq!(Solution::is_possible(4, vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 4]]), true);
}

#[test]
fn test_is_possible5() {
    assert_eq!(Solution::is_possible(5, vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]]), true);
}

#[test]
fn test_is_possible6() {
    assert_eq!(Solution::is_possible(4, vec![vec![4, 1], vec![3, 2], vec![2, 4], vec![1, 3]]), true);
}
