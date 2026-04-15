// Tests for Problem 3493: Properties Graph
// Java reference: src/test/java/g3401_3500/s3493_properties_graph/SolutionTest.java

use leetcode_in_rust::s3493::properties_graph::Solution;

#[test]
fn test_number_of_components() {
    assert_eq!(
        Solution::number_of_components(vec![vec![1, 2], vec![1, 1], vec![3, 4], vec![4, 5], vec![5, 6], vec![7, 7]], 1),
        3
    );
}

#[test]
fn test_number_of_components2() {
    assert_eq!(
        Solution::number_of_components(vec![vec![1, 2, 3], vec![2, 3, 4], vec![4, 3, 5]], 2),
        1
    );
}

#[test]
fn test_number_of_components3() {
    assert_eq!(
        Solution::number_of_components(vec![vec![1, 1], vec![1, 1]], 2),
        2
    );
}
