// Tests for Problem 1691: Maximum Height by Stacking Cuboids
// Java reference: src/test/java/g1601_1700/s1691_maximum_height_by_stacking_cuboids/SolutionTest.java

use leetcode_in_rust::s1691::maximum_height_by_stacking_cuboids::Solution;

#[test]
fn test_max_height() {
    assert_eq!(
        Solution::max_height(vec![vec![50, 45, 20], vec![95, 37, 53], vec![45, 23, 12]]),
        190
    );
}

#[test]
fn test_max_height2() {
    assert_eq!(Solution::max_height(vec![vec![38, 25, 45], vec![76, 35, 3]]), 76);
}

#[test]
fn test_max_height3() {
    assert_eq!(
        Solution::max_height(vec![
            vec![7, 11, 17],
            vec![7, 17, 11],
            vec![11, 7, 17],
            vec![11, 17, 7],
            vec![17, 7, 11],
            vec![17, 11, 7]
        ]),
        102
    );
}
