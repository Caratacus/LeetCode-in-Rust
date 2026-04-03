// Tests for Problem 1782: Count Pairs of Nodes
// Java reference: src/test/java/g1701_1800/s1782_count_pairs_of_nodes/SolutionTest.java

use leetcode_in_rust::s1782::count_pairs_of_nodes::Solution;

#[test]
fn test_count_pairs() {
    assert_eq!(
        Solution::count_pairs(
            4,
            vec![vec![1, 2], vec![2, 4], vec![1, 3], vec![2, 3], vec![2, 1]],
            vec![2, 3]
        ),
        vec![6, 5]
    );
}

#[test]
fn test_count_pairs2() {
    assert_eq!(
        Solution::count_pairs(
            5,
            vec![
                vec![1, 5], vec![1, 5], vec![3, 4], vec![2, 5],
                vec![1, 3], vec![5, 1], vec![2, 3], vec![2, 5]
            ],
            vec![1, 2, 3, 4, 5]
        ),
        vec![10, 10, 9, 8, 6]
    );
}
