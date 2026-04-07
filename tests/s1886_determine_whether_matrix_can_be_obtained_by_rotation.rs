// Tests for Problem 1886: Determine Whether Matrix Can Be Obtained By Rotation
// Java reference: src/test/java/g1801_1900/s1886_determine_whether_matrix_can_be_obtained_by_rotation/SolutionTest.java

use leetcode_in_rust::s1886::determine_whether_matrix_can_be_obtained_by_rotation::Solution;

#[test]
fn test_find_rotation() {
    assert_eq!(
        Solution::find_rotation(vec![vec![0, 1], vec![1, 0]], vec![vec![1, 0], vec![0, 1]]),
        true
    );
}

#[test]
fn test_find_rotation2() {
    assert_eq!(
        Solution::find_rotation(vec![vec![0, 1], vec![1, 1]], vec![vec![1, 0], vec![0, 1]]),
        false
    );
}

#[test]
fn test_find_rotation3() {
    assert_eq!(
        Solution::find_rotation(
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]],
            vec![vec![1, 1, 1], vec![0, 1, 0], vec![0, 0, 0]]
        ),
        true
    );
}
