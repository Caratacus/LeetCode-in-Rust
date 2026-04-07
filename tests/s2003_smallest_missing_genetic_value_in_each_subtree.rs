// Tests for Problem 2003: Smallest Missing Genetic Value in Each Subtree
// Java reference: src/test/java/g2001_2100/s2003_smallest_missing_genetic_value_in_each_subtree/SolutionTest.java

use leetcode_in_rust::s2003::smallest_missing_genetic_value_in_each_subtree::Solution;

#[test]
fn test_smallest_missing_value_subtree() {
    assert_eq!(
        Solution::smallest_missing_value_subtree(vec![-1, 0, 0, 2], vec![1, 2, 3, 4]),
        vec![5, 1, 1, 1]
    );
}

#[test]
fn test_smallest_missing_value_subtree2() {
    assert_eq!(
        Solution::smallest_missing_value_subtree(vec![-1, 0, 1, 0, 3, 3], vec![5, 4, 6, 2, 1, 3]),
        vec![7, 1, 1, 4, 2, 1]
    );
}

#[test]
fn test_smallest_missing_value_subtree3() {
    assert_eq!(
        Solution::smallest_missing_value_subtree(vec![-1, 2, 3, 0, 2, 4, 1], vec![2, 3, 4, 5, 6, 7, 8]),
        vec![1, 1, 1, 1, 1, 1, 1]
    );
}
