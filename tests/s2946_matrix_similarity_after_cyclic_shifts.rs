// Tests for Problem 2946: Matrix Similarity After Cyclic Shifts
// Java reference: src/test/java/g2901_3000/s2946_matrix_similarity_after_cyclic_shifts/SolutionTest.java

use leetcode_in_rust::s2946::matrix_similarity_after_cyclic_shifts::Solution;

#[test]
fn test_are_similar() {
    assert_eq!(
        Solution::are_similar(vec![vec![1, 2, 1, 2], vec![5, 5, 5, 5], vec![6, 3, 6, 3]], 2),
        true
    );
}

#[test]
fn test_are_similar2() {
    assert_eq!(Solution::are_similar(vec![vec![2, 2], vec![2, 2]], 3), true);
}

#[test]
fn test_are_similar3() {
    assert_eq!(Solution::are_similar(vec![vec![1, 2]], 1), false);
}
