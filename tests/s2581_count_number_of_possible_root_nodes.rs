// Tests for Problem 2581: Count Number of Possible Root Nodes
// Java reference: src/test/java/g2501_2600/s2581_count_number_of_possible_root_nodes/SolutionTest.java

use leetcode_in_rust::s2581::count_number_of_possible_root_nodes::Solution;

#[test]
fn test_root_count() {
    assert_eq!(
        Solution::root_count(
            vec![vec![0, 1], vec![1, 2], vec![1, 3], vec![4, 2]],
            vec![vec![1, 3], vec![0, 1], vec![1, 0], vec![2, 4]],
            3
        ),
        3
    );
}

#[test]
fn test_root_count2() {
    assert_eq!(
        Solution::root_count(
            vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4]],
            vec![vec![1, 0], vec![3, 4], vec![2, 1], vec![3, 2]],
            1
        ),
        5
    );
}
