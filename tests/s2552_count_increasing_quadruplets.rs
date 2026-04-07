// Tests for Problem 2552: Count Increasing Quadruplets
// Java reference: src/test/java/g2501_2600/s2552_count_increasing_quadruplets/SolutionTest.java
use leetcode_in_rust::s2552::count_increasing_quadruplets::Solution;

#[test]
fn test_count_quadruplets() {
    assert_eq!(Solution::count_quadruplets(vec![1, 3, 2, 4, 5]), 2);
}
#[test]
fn test_count_quadruplets2() {
    assert_eq!(Solution::count_quadruplets(vec![1, 2, 3, 4, 5]), 2);
}
