// Tests for Problem 1131: Maximum of Absolute Value Expression
// Java reference: src/test/java/g1101_1200/s1131_maximum_of_absolute_value_expression/SolutionTest.java

use leetcode_in_rust::s1131::maximum_of_absolute_value_expression::Solution;

#[test]
fn test_max_abs_val_expr() {
    assert_eq!(
        Solution::max_abs_val_expr(vec![1, 2, 3, 4], vec![-1, 4, 5, 6]),
        13
    );
}

#[test]
fn test_max_abs_val_expr2() {
    assert_eq!(
        Solution::max_abs_val_expr(vec![1, -2, -5, 0, 10], vec![0, -2, -1, -7, -4]),
        20
    );
}
