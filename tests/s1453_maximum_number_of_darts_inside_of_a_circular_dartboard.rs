// Tests for Problem 1453: Maximum Number of Darts Inside of a Circular Dartboard
// Java reference: src/test/java/g1401_1500/s1453_maximum_number_of_darts_inside_of_a_circular_dartboard/SolutionTest.java

use leetcode_in_rust::s1453::maximum_number_of_darts_inside_of_a_circular_dartboard::Solution;

#[test]
fn test_num_points() {
    let points = vec![vec![-2, 0], vec![2, 0], vec![0, 2], vec![0, -2]];
    assert_eq!(Solution::num_points(points, 2), 4);
}

#[test]
fn test_num_points2() {
    let points = vec![vec![-3, 0], vec![3, 0], vec![2, 6], vec![5, 4], vec![0, 9], vec![7, 8]];
    assert_eq!(Solution::num_points(points, 5), 5);
}
