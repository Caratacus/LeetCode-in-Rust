// Tests for Problem 0498: Diagonal Traverse
// Java reference: src/test/java/g0401_0500/s0498_diagonal_traverse/SolutionTest.java

use leetcode_in_rust::s0498::diagonal_traverse::Solution;

#[test]
fn test_find_diagonal_order() {
    assert_eq!(
        Solution::find_diagonal_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]),
        vec![1, 2, 4, 7, 5, 3, 6, 8, 9]
    );
}

#[test]
fn test_find_diagonal_order2() {
    assert_eq!(
        Solution::find_diagonal_order(vec![vec![1, 2], vec![3, 4]]),
        vec![1, 2, 3, 4]
    );
}
