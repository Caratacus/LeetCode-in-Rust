// Tests for Problem 2171: Removing Minimum Number of Magic Beans
// Java reference: src/test/java/g2101_2200/s2171_removing_minimum_number_of_magic_beans/SolutionTest.java

use leetcode_in_rust::s2171::removing_minimum_number_of_magic_beans::Solution;

#[test]
fn test_minimum_removal() {
    assert_eq!(Solution::minimum_removal(vec![4, 1, 6, 5]), 4);
}

#[test]
fn test_minimum_removal2() {
    assert_eq!(Solution::minimum_removal(vec![2, 10, 3, 2]), 7);
}
