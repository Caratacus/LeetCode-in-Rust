// Tests for Problem 3380: Maximum Area Rectangle With Point Constraints I
// Java reference: src/test/java/g3301_3400/s3380_maximum_area_rectangle_with_point_constraints_i/SolutionTest.java

use leetcode_in_rust::s3380::maximum_area_rectangle_with_point_constraints_i::Solution;

#[test]
fn test_max_rectangle_area() {
    assert_eq!(
        Solution::max_rectangle_area(vec![vec![1, 1], vec![1, 3], vec![3, 1], vec![3, 3]]),
        4
    );
}

#[test]
fn test_max_rectangle_area2() {
    assert_eq!(
        Solution::max_rectangle_area(vec![
            vec![1, 1],
            vec![1, 3],
            vec![3, 1],
            vec![3, 3],
            vec![2, 2]
        ]),
        -1
    );
}

#[test]
fn test_max_rectangle_area3() {
    assert_eq!(
        Solution::max_rectangle_area(vec![
            vec![1, 1],
            vec![1, 3],
            vec![3, 1],
            vec![3, 3],
            vec![1, 2],
            vec![3, 2]
        ]),
        2
    );
}
