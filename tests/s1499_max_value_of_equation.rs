// Tests for Problem 1499: Max Value of Equation
// Java reference: src/test/java/g1401_1500/s1499_max_value_of_equation/SolutionTest.java

use leetcode_in_rust::s1499::max_value_of_equation::Solution;

#[test]
fn test_find_max_value_of_equation() {
    let points = vec![vec![1, 3], vec![2, 0], vec![5, 10], vec![6, -10]];
    assert_eq!(Solution::find_max_value_of_equation(points, 1), 4);
}

#[test]
fn test_find_max_value_of_equation2() {
    let points = vec![vec![0, 0], vec![3, 0], vec![9, 2]];
    assert_eq!(Solution::find_max_value_of_equation(points, 3), 3);
}
