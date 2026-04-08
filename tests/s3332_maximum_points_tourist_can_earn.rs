// Tests for Problem 3332: Maximum Points Tourist Can Earn
// Java reference: src/test/java/g3301_3400/s3332_maximum_points_tourist_can_earn/SolutionTest.java

use leetcode_in_rust::s3332::maximum_points_tourist_can_earn::Solution;

#[test]
fn test_max_score() {
    assert_eq!(
        Solution::max_score(2, 1, vec![vec![2, 3]], vec![vec![0, 2], vec![1, 0]]),
        3
    );
}

#[test]
fn test_max_score2() {
    assert_eq!(
        Solution::max_score(
            3,
            2,
            vec![vec![3, 4, 2], vec![2, 1, 2]],
            vec![vec![0, 2, 1], vec![2, 0, 4], vec![3, 2, 0]]
        ),
        8
    );
}
