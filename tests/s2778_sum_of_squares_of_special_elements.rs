// Tests for Problem 2778: Sum of Squares of Special Elements
// Java reference: src/test/java/g2701_2800/s2778_sum_of_squares_of_special_elements/SolutionTest.java

use leetcode_in_rust::s2778::sum_of_squares_of_special_elements::Solution;

#[test]
fn test_sum_of_squares() {
    assert_eq!(Solution::sum_of_squares(vec![1, 2, 3, 4]), 21);
}

#[test]
fn test_sum_of_squares2() {
    assert_eq!(Solution::sum_of_squares(vec![2, 7, 1, 19, 18, 3]), 63);
}
