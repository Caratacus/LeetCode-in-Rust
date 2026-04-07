// Tests for Problem 2513: Minimize the Maximum of Two Arrays
// Java reference: src/test/java/g2401_2500/s2513_minimize_the_maximum_of_two_arrays/SolutionTest.java

use leetcode_in_rust::s2513::minimize_the_maximum_of_two_arrays::Solution;

#[test]
fn test_minimize_set() {
    assert_eq!(Solution::minimize_set(2, 7, 1, 3), 4);
}

#[test]
fn test_minimize_set2() {
    assert_eq!(Solution::minimize_set(3, 5, 2, 1), 3);
}
