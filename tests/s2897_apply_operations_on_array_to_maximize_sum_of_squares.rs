// Tests for Problem 2897: Apply Operations on Array to Maximize Sum of Squares
// Java reference: src/test/java/g2801_2900/s2897_apply_operations_on_array_to_maximize_sum_of_squares/SolutionTest.java

use leetcode_in_rust::s2897::apply_operations_on_array_to_maximize_sum_of_squares::Solution;

#[test]
fn test_max_sum() {
    assert_eq!(Solution::max_sum(vec![2, 6, 5, 8], 2), 261);
}

#[test]
fn test_max_sum2() {
    assert_eq!(Solution::max_sum(vec![4, 5, 4, 7], 3), 90);
}
