// Tests for Problem 2672: Number of Adjacent Elements With the Same Color
// Java reference: src/test/java/g2601_2700/s2672_number_of_adjacent_elements_with_the_same_color/SolutionTest.java

use leetcode_in_rust::s2672::number_of_adjacent_elements_with_the_same_color::Solution;

#[test]
fn test_color_the_array() {
    assert_eq!(
        Solution::color_the_array(
            4,
            vec![vec![0, 2], vec![1, 2], vec![3, 1], vec![1, 1], vec![2, 1]]
        ),
        vec![0, 1, 1, 0, 2]
    );
}

#[test]
fn test_color_the_array2() {
    assert_eq!(
        Solution::color_the_array(1, vec![vec![0, 100000]]),
        vec![0]
    );
}
