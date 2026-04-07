// Tests for Problem 2824: Count Pairs Whose Sum is Less than Target
// Java reference: src/test/java/g2801_2900/s2824_count_pairs_whose_sum_is_less_than_target/SolutionTest.java

use leetcode_in_rust::s2824::count_pairs_whose_sum_is_less_than_target::Solution;

#[test]
fn test_count_pairs() {
    assert_eq!(Solution::count_pairs(vec![-1, 1, 2, 3, 1], 2), 3);
}

#[test]
fn test_count_pairs2() {
    assert_eq!(Solution::count_pairs(vec![-6, 2, 5, -2, -7, -1, 3], -2), 10);
}
