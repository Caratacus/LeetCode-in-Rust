// Tests for Problem 1995: Count Special Quadruplets
// Java reference: src/test/java/g1901_2000/s1995_count_special_quadruplets/SolutionTest.java

use leetcode_in_rust::s1995::count_special_quadruplets::Solution;

#[test]
fn test_count_quadruplets() {
    assert_eq!(Solution::count_quadruplets(vec![1, 2, 3, 6]), 1);
}

#[test]
fn test_count_quadruplets2() {
    assert_eq!(Solution::count_quadruplets(vec![3, 3, 6, 4, 5]), 0);
}

#[test]
fn test_count_quadruplets3() {
    assert_eq!(Solution::count_quadruplets(vec![1, 1, 1, 3, 5]), 4);
}
