// Tests for Problem 2920: Maximum Points After Collecting Coins From All Nodes
// Java reference: src/test/java/g2901_3000/s2920_maximum_points_after_collecting_coins_from_all_nodes/SolutionTest.java

use leetcode_in_rust::s2920::maximum_points_after_collecting_coins_from_all_nodes::Solution;

#[test]
fn test_maximum_points() {
    assert_eq!(
        Solution::maximum_points(
            vec![vec![0, 1], vec![1, 2], vec![2, 3]],
            vec![10, 10, 3, 3],
            5
        ),
        11
    );
}

#[test]
fn test_maximum_points2() {
    assert_eq!(
        Solution::maximum_points(
            vec![vec![0, 1], vec![0, 2]],
            vec![8, 4, 4],
            0
        ),
        16
    );
}
