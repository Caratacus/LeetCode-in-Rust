// Tests for Problem 2643: Row With Maximum Ones
// Java reference: src/test/java/g2601_2700/s2643_row_with_maximum_ones/SolutionTest.java

use leetcode_in_rust::s2643::row_with_maximum_ones::Solution;

#[test]
fn test_row_and_maximum_ones() {
    assert_eq!(
        Solution::row_and_maximum_ones(vec![vec![0, 1], vec![1, 0]]),
        vec![0, 1]
    );
}

#[test]
fn test_row_and_maximum_ones2() {
    assert_eq!(
        Solution::row_and_maximum_ones(vec![vec![0, 0, 0], vec![0, 1, 1]]),
        vec![1, 2]
    );
}

#[test]
fn test_row_and_maximum_ones3() {
    assert_eq!(
        Solution::row_and_maximum_ones(vec![vec![0, 0], vec![1, 1], vec![0, 0]]),
        vec![1, 2]
    );
}
