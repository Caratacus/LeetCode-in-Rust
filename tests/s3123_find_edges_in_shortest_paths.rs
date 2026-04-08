// Tests for Problem 3123: Find Edges in Shortest Paths
// Java reference: src/test/java/g3101_3200/s3123_find_edges_in_shortest_paths/SolutionTest.java

use leetcode_in_rust::s3123::find_edges_in_shortest_paths::Solution;

#[test]
fn test_find_answer() {
    assert_eq!(
        Solution::find_answer(
            6,
            vec![
                vec![0, 1, 4],
                vec![0, 2, 1],
                vec![1, 3, 2],
                vec![1, 4, 3],
                vec![1, 5, 1],
                vec![2, 3, 1],
                vec![3, 5, 3],
                vec![4, 5, 2]
            ]
        ),
        vec![true, true, true, false, true, true, true, false]
    );
}

#[test]
fn test_find_answer2() {
    assert_eq!(
        Solution::find_answer(
            4,
            vec![vec![2, 0, 1], vec![0, 1, 1], vec![0, 3, 4], vec![3, 2, 2]]
        ),
        vec![true, false, false, true]
    );
}
