// Tests for Problem 2250: Count Number of Rectangles Containing Each Point
// Java reference: src/test/java/g2201_2300/s2250_count_number_of_rectangles_containing_each_point/SolutionTest.java

use leetcode_in_rust::s2250::count_number_of_rectangles_containing_each_point::Solution;

#[test]
fn test_count_rectangles() {
    assert_eq!(
        Solution::count_rectangles(
            vec![vec![1, 2], vec![2, 3], vec![2, 5]],
            vec![vec![2, 1], vec![1, 4]]
        ),
        vec![2, 1]
    );
}

#[test]
fn test_count_rectangles2() {
    assert_eq!(
        Solution::count_rectangles(
            vec![vec![1, 1], vec![2, 2], vec![3, 3]],
            vec![vec![1, 3], vec![1, 1]]
        ),
        vec![1, 3]
    );
}
