// Tests for Problem 3225: Maximum Score From Grid Operations
// Java reference: src/test/java/g3201_3300/s3225_maximum_score_from_grid_operations/SolutionTest.java

use leetcode_in_rust::s3225::maximum_score_from_grid_operations::Solution;

#[test]
fn test_maximum_score() {
    assert_eq!(
        Solution::maximum_score(vec![
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 3, 0, 0],
            vec![0, 1, 0, 0, 0],
            vec![5, 0, 0, 3, 0],
            vec![0, 0, 0, 0, 2]
        ]),
        11
    );
}

#[test]
fn test_maximum_score2() {
    assert_eq!(
        Solution::maximum_score(vec![
            vec![10, 9, 0, 0, 15],
            vec![7, 1, 0, 8, 0],
            vec![5, 20, 0, 11, 0],
            vec![0, 0, 0, 1, 2],
            vec![8, 12, 1, 10, 3]
        ]),
        94
    );
}
