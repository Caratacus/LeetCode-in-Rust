// Tests for Problem 1129: Shortest Path with Alternating Colors
// Java reference: src/test/java/g1101_1200/s1129_shortest_path_with_alternating_colors/SolutionTest.java

use leetcode_in_rust::s1129::shortest_path_with_alternating_colors::Solution;

#[test]
fn test_shortest_alternating_paths() {
    assert_eq!(
        Solution::shortest_alternating_paths(3, vec![vec![0, 1], vec![1, 2]], vec![]),
        vec![0, 1, -1]
    );
}

#[test]
fn test_shortest_alternating_paths2() {
    assert_eq!(
        Solution::shortest_alternating_paths(3, vec![vec![0, 1]], vec![vec![2, 1]]),
        vec![0, 1, -1]
    );
}
