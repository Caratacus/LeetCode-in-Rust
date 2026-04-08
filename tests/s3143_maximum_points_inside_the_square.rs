// Tests for Problem 3143: Maximum Points Inside the Square
// Java reference: src/test/java/g3101_3200/s3143_maximum_points_inside_the_square/SolutionTest.java

use leetcode_in_rust::s3143::maximum_points_inside_the_square::Solution;

#[test]
fn test_max_points_inside_square() {
    assert_eq!(
        Solution::max_points_inside_square(
            vec![vec![2, 2], vec![-1, -2], vec![-4, 4], vec![-3, 1], vec![3, -3]],
            "abdca".to_string()
        ),
        2
    );
}

#[test]
fn test_max_points_inside_square2() {
    assert_eq!(
        Solution::max_points_inside_square(
            vec![vec![1, 1], vec![-2, -2], vec![-2, 2]],
            "abb".to_string()
        ),
        1
    );
}
