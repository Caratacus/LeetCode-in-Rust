// Tests for Problem 3620: Network Recovery Pathways
// Java reference: src/test/java/g3601_3700/s3620_network_recovery_pathways/SolutionTest.java
use leetcode_in_rust::s3620::network_recovery_pathways::Solution;
#[test]
fn test_find_max_path_score() {
    assert_eq!(
        Solution::find_max_path_score(
            vec![vec![0, 1, 5], vec![1, 3, 10], vec![0, 2, 3], vec![2, 3, 4]],
            vec![true, true, true, true],
            10i64
        ),
        3
    );
}
#[test]
fn test_find_max_path_score2() {
    assert_eq!(
        Solution::find_max_path_score(
            vec![vec![0, 1, 7], vec![1, 4, 5], vec![0, 2, 6], vec![2, 3, 6], vec![3, 4, 2], vec![2, 4, 6]],
            vec![true, true, true, false, true],
            12i64
        ),
        6
    );
}
