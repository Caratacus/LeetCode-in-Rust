// Tests for Problem 1519: Number of Nodes in the Sub-Tree with the Same Label
// Java reference: src/test/java/g1501_1600/s1519_number_of_nodes_in_the_sub_tree_with_the_same_label/SolutionTest.java

use leetcode_in_rust::s1519::number_of_nodes_in_the_sub_tree_with_the_same_label::Solution;

#[test]
fn test_count_sub_trees() {
    assert_eq!(
        Solution::count_sub_trees(
            7,
            vec![vec![0, 1], vec![0, 2], vec![1, 4], vec![1, 5], vec![2, 3], vec![2, 6]],
            "abaedcd".to_string()
        ),
        vec![2, 1, 1, 1, 1, 1, 1]
    );
}

#[test]
fn test_count_sub_trees2() {
    assert_eq!(
        Solution::count_sub_trees(
            4,
            vec![vec![0, 1], vec![1, 2], vec![0, 3]],
            "bbbb".to_string()
        ),
        vec![4, 2, 1, 1]
    );
}

#[test]
fn test_count_sub_trees3() {
    assert_eq!(
        Solution::count_sub_trees(
            5,
            vec![vec![0, 1], vec![0, 2], vec![1, 3], vec![0, 4]],
            "aabab".to_string()
        ),
        vec![3, 2, 1, 1, 1]
    );
}
