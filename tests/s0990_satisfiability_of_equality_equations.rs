// Tests for Problem 0990: Satisfiability of Equality Equations
// Java reference: src/test/java/g0901_1000/s0990_satisfiability_of_equality_equations/SolutionTest.java

use leetcode_in_rust::s0990::satisfiability_of_equality_equations::Solution;

#[test]
fn test_equations_possible() {
    assert_eq!(
        Solution::equations_possible(vec!["a==b".to_string(), "b!=a".to_string()]),
        false
    );
}

#[test]
fn test_equations_possible2() {
    assert_eq!(
        Solution::equations_possible(vec!["b==a".to_string(), "a==b".to_string()]),
        true
    );
}
