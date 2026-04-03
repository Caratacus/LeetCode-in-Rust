// Tests for Problem 1584: Min Cost to Connect All Points
// Java reference: src/test/java/g1501_1600/s1584_min_cost_to_connect_all_points/SolutionTest.java

use leetcode_in_rust::s1584::min_cost_to_connect_all_points::Solution;

#[test]
fn test_min_cost_connect_points() {
    assert_eq!(
        Solution::min_cost_connect_points(vec![vec![0, 0], vec![2, 2], vec![3, 10], vec![5, 2], vec![7, 0]]),
        20
    );
}

#[test]
fn test_min_cost_connect_points2() {
    assert_eq!(
        Solution::min_cost_connect_points(vec![vec![3, 12], vec![-2, 5], vec![-4, 1]]),
        18
    );
}
