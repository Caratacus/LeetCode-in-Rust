// Tests for Problem 2616: Minimize the Maximum Difference of Pairs
// Java reference: src/test/java/g2601_2700/s2616_minimize_the_maximum_difference_of_pairs/SolutionTest.java

use leetcode_in_rust::s2616::minimize_the_maximum_difference_of_pairs::Solution;

#[test]
fn test_minimize_max() {
    assert_eq!(Solution::minimize_max(vec![10, 1, 2, 7, 1, 3], 2), 1);
}

#[test]
fn test_minimize_max2() {
    assert_eq!(Solution::minimize_max(vec![4, 2, 1, 2], 1), 0);
}
