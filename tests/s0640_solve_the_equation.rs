// Tests for Problem 0640: Solve the Equation
// Java reference: src/test/java/g0601_0700/s0640_solve_the_equation/SolutionTest.java

use leetcode_in_rust::s0640::solve_the_equation::Solution;

#[test]
fn test_solve_equation() {
    assert_eq!(
        Solution::solve_equation("x+5-3+x=6+x-2".to_string()),
        "x=2"
    );
}

#[test]
fn test_solve_equation2() {
    assert_eq!(
        Solution::solve_equation("x=x".to_string()),
        "Infinite solutions"
    );
}

#[test]
fn test_solve_equation3() {
    assert_eq!(Solution::solve_equation("2x=x".to_string()), "x=0");
}
