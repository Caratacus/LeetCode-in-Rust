// Tests for Problem 3111: Minimum Rectangles to Cover Points
// Java reference: src/test/java/g3101_3200/s3111_minimum_rectangles_to_cover_points/SolutionTest.java

use leetcode_in_rust::s3111::minimum_rectangles_to_cover_points::Solution;

#[test]
fn test_min_rectangles_to_cover_points() {
    assert_eq!(
        Solution::min_rectangles_to_cover_points(
            vec![vec![2, 1], vec![1, 0], vec![1, 4], vec![1, 8], vec![3, 5], vec![4, 6]],
            1
        ),
        2
    );
}

#[test]
fn test_min_rectangles_to_cover_points2() {
    assert_eq!(
        Solution::min_rectangles_to_cover_points(
            vec![
                vec![0, 0],
                vec![1, 1],
                vec![2, 2],
                vec![3, 3],
                vec![4, 4],
                vec![5, 5],
                vec![6, 6]
            ],
            2
        ),
        3
    );
}

#[test]
fn test_min_rectangles_to_cover_points3() {
    assert_eq!(
        Solution::min_rectangles_to_cover_points(vec![vec![2, 3], vec![1, 2]], 0),
        2
    );
}
