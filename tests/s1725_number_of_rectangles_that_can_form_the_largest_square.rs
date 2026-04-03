// Tests for Problem 1725: Number of Rectangles That Can Form the Largest Square
// Java reference: src/test/java/g1701_1800/s1725_number_of_rectangles_that_can_form_the_largest_square/SolutionTest.java

use leetcode_in_rust::s1725::number_of_rectangles_that_can_form_the_largest_square::Solution;

#[test]
fn test_count_good_rectangles() {
    assert_eq!(
        Solution::count_good_rectangles(vec![vec![5, 8], vec![3, 9], vec![5, 12], vec![16, 5]]),
        3
    );
}

#[test]
fn test_count_good_rectangles2() {
    assert_eq!(
        Solution::count_good_rectangles(vec![vec![2, 3], vec![3, 7], vec![4, 3], vec![3, 7]]),
        3
    );
}
