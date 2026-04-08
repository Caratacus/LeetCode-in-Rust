// Tests for Problem 3128: Right Triangles
// Java reference: src/test/java/g3101_3200/s3128_right_triangles/SolutionTest.java

use leetcode_in_rust::s3128::right_triangles::Solution;

#[test]
fn test_number_of_right_triangles() {
    assert_eq!(
        Solution::number_of_right_triangles(vec![vec![0, 1, 0], vec![0, 1, 1], vec![0, 1, 0]]),
        2
    );
}

#[test]
fn test_number_of_right_triangles2() {
    assert_eq!(
        Solution::number_of_right_triangles(vec![
            vec![1, 0, 0, 0],
            vec![0, 1, 0, 1],
            vec![1, 0, 0, 0]
        ]),
        0
    );
}

#[test]
fn test_number_of_right_triangles3() {
    assert_eq!(
        Solution::number_of_right_triangles(vec![vec![1, 0, 1], vec![1, 0, 0], vec![1, 0, 0]]),
        2
    );
}
