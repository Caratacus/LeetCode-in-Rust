// Tests for Problem 3382: Maximum Area Rectangle With Point Constraints II
// Java reference: src/test/java/g3301_3400/s3382_maximum_area_rectangle_with_point_constraints_ii/SolutionTest.java

use leetcode_in_rust::s3382::maximum_area_rectangle_with_point_constraints_ii::Solution;

#[test]
fn test_max_rectangle_area() {
    assert_eq!(
        Solution::max_rectangle_area(vec![1, 1, 3, 3], vec![1, 3, 1, 3]),
        4
    );
}

#[test]
fn test_max_rectangle_area2() {
    assert_eq!(
        Solution::max_rectangle_area(vec![1, 1, 3, 3, 2], vec![1, 3, 1, 3, 2]),
        -1
    );
}

#[test]
fn test_max_rectangle_area3() {
    assert_eq!(
        Solution::max_rectangle_area(vec![1, 1, 3, 3, 1, 3], vec![1, 3, 1, 3, 2, 2]),
        2
    );
}
