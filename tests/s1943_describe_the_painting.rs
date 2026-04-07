// Tests for Problem 1943: Describe the Painting
// Java reference: src/test/java/g1901_2000/s1943_describe_the_painting/SolutionTest.java

use leetcode_in_rust::s1943::describe_the_painting::Solution;

#[test]
fn test_split_painting() {
    assert_eq!(
        Solution::split_painting(vec![vec![1, 4, 5], vec![4, 7, 7], vec![1, 7, 9]]),
        vec![vec![1, 4, 14], vec![4, 7, 16]]
    );
}

#[test]
fn test_split_painting2() {
    assert_eq!(
        Solution::split_painting(vec![vec![1, 7, 9], vec![6, 8, 15], vec![8, 10, 7]]),
        vec![vec![1, 6, 9], vec![6, 7, 24], vec![7, 8, 15], vec![8, 10, 7]]
    );
}

#[test]
fn test_split_painting3() {
    assert_eq!(
        Solution::split_painting(vec![vec![1, 4, 5], vec![1, 4, 7], vec![4, 7, 1], vec![4, 7, 11]]),
        vec![vec![1, 4, 12], vec![4, 7, 12]]
    );
}
