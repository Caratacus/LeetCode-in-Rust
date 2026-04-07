// Tests for Problem 1862: Sum of Floored Pairs
// Java reference: src/test/java/g1801_1900/s1862_sum_of_floored_pairs/SolutionTest.java

use leetcode_in_rust::s1862::sum_of_floored_pairs::Solution;

#[test]
fn test_sum_of_floored_pairs() {
    assert_eq!(Solution::sum_of_floored_pairs(vec![2, 5, 9]), 10);
}

#[test]
fn test_sum_of_floored_pairs2() {
    assert_eq!(
        Solution::sum_of_floored_pairs(vec![7, 7, 7, 7, 7, 7, 7]),
        49
    );
}
