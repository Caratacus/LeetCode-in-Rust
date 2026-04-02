// Tests for Problem 0457: Circular Array Loop
// Java reference: src/test/java/g0401_0500/s0457_circular_array_loop/SolutionTest.java

use leetcode_in_rust::s0457::circular_array_loop::Solution;

#[test]
fn test_circular_array_loop() {
    assert_eq!(Solution::circular_array_loop(vec![2, -1, 1, 2, 2]), true);
}

#[test]
fn test_circular_array_loop2() {
    assert_eq!(Solution::circular_array_loop(vec![-1, 2]), false);
}

#[test]
fn test_circular_array_loop3() {
    assert_eq!(Solution::circular_array_loop(vec![-2, 1, -1, -2, -2]), false);
}
