// Tests for Problem 1617: Count Subtrees With Max Distance Between Cities
// Java reference: src/test/java/g1601_1700/s1617_count_subtrees_with_max_distance_between_cities/SolutionTest.java

use leetcode_in_rust::s1617::count_subtrees_with_max_distance_between_cities::Solution;

#[test]
fn test_count_subgraphs_for_each_diameter() {
    assert_eq!(
        Solution::count_subgraphs_for_each_diameter(4, vec![vec![1, 2], vec![2, 3], vec![2, 4]]),
        vec![3, 4, 0]
    );
}

#[test]
fn test_count_subgraphs_for_each_diameter2() {
    assert_eq!(
        Solution::count_subgraphs_for_each_diameter(2, vec![vec![1, 2]]),
        vec![1]
    );
}

#[test]
fn test_count_subgraphs_for_each_diameter3() {
    assert_eq!(
        Solution::count_subgraphs_for_each_diameter(3, vec![vec![1, 2], vec![2, 3]]),
        vec![2, 1]
    );
}
