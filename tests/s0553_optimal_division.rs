// Tests for Problem 0553: Optimal Division
// Java reference: src/test/java/g0501_0600/s0553_optimal_division/SolutionTest.java

use leetcode_in_rust::s0553::optimal_division::Solution;

#[test]
fn test_optimal_division() {
    assert_eq!(
        Solution::optimal_division(vec![1000, 100, 10, 2]),
        "1000/(100/10/2)"
    );
}

#[test]
fn test_optimal_division2() {
    assert_eq!(Solution::optimal_division(vec![2, 3, 4]), "2/(3/4)");
}

#[test]
fn test_optimal_division3() {
    assert_eq!(Solution::optimal_division(vec![2]), "2");
}
