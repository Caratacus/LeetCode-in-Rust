// Tests for Problem 2826: Sorting Three Groups
// Java reference: src/test/java/g2801_2900/s2826_sorting_three_groups/SolutionTest.java

use leetcode_in_rust::s2826::sorting_three_groups::Solution;

#[test]
fn test_minimum_operations() {
    assert_eq!(Solution::minimum_operations(vec![2, 1, 3, 2, 1]), 3);
}

#[test]
fn test_minimum_operations2() {
    assert_eq!(Solution::minimum_operations(vec![1, 2, 2, 1]), 1);
}
