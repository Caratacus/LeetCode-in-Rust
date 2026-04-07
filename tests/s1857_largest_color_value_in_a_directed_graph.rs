// Tests for Problem 1857: Largest Color Value in a Directed Graph
// Java reference: src/test/java/g1801_1900/s1857_largest_color_value_in_a_directed_graph/SolutionTest.java

use leetcode_in_rust::s1857::largest_color_value_in_a_directed_graph::Solution;

#[test]
fn test_largest_path_value() {
    assert_eq!(
        Solution::largest_path_value(
            "abaca".to_string(),
            vec![vec![0, 1], vec![0, 2], vec![2, 3], vec![3, 4]]
        ),
        3
    );
}

#[test]
fn test_largest_path_value2() {
    assert_eq!(
        Solution::largest_path_value("a".to_string(), vec![vec![0, 0]]),
        -1
    );
}
