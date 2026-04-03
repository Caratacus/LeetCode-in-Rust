// Tests for Problem 1786: Number of Restricted Paths From First to Last Node
// Java reference: src/test/java/g1701_1800/s1786_number_of_restricted_paths_from_first_to_last_node/SolutionTest.java

use leetcode_in_rust::s1786::number_of_restricted_paths_from_first_to_last_node::Solution;

#[test]
fn test_count_restricted_paths() {
    assert_eq!(
        Solution::count_restricted_paths(
            5,
            vec![
                vec![1, 2, 3],
                vec![1, 3, 3],
                vec![2, 3, 1],
                vec![1, 4, 2],
                vec![5, 2, 2],
                vec![3, 5, 1],
                vec![5, 4, 10]
            ]
        ),
        3
    );
}

#[test]
fn test_count_restricted_paths2() {
    assert_eq!(
        Solution::count_restricted_paths(
            7,
            vec![
                vec![1, 3, 1],
                vec![4, 1, 2],
                vec![7, 3, 4],
                vec![2, 5, 3],
                vec![5, 6, 1],
                vec![6, 7, 2],
                vec![7, 5, 3],
                vec![2, 6, 4]
            ]
        ),
        1
    );
}
