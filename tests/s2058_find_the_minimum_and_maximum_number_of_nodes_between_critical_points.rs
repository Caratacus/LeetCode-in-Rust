// Tests for Problem 2058: Find the Minimum and Maximum Number of Nodes Between Critical Points
// Java reference: src/test/java/g2001_2100/s2058_find_the_minimum_and_maximum_number_of_nodes_between_critical_points/SolutionTest.java

use leetcode_in_rust::s2058::find_the_minimum_and_maximum_number_of_nodes_between_critical_points::Solution;
use leetcode_in_rust::utils::linked_list_utils::linked_list_from_vec;

#[test]
fn test_nodes_between_critical_points() {
    let head = linked_list_from_vec(vec![3, 1]);
    assert_eq!(Solution::nodes_between_critical_points(head), vec![-1, -1]);
}

#[test]
fn test_nodes_between_critical_points2() {
    let head = linked_list_from_vec(vec![5, 3, 1, 2, 5, 1, 2]);
    assert_eq!(Solution::nodes_between_critical_points(head), vec![1, 3]);
}

#[test]
fn test_nodes_between_critical_points3() {
    let head = linked_list_from_vec(vec![1, 3, 2, 2, 3, 2, 2, 2, 7]);
    assert_eq!(Solution::nodes_between_critical_points(head), vec![3, 3]);
}
