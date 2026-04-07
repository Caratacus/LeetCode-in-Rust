// Tests for Problem 2856: Minimum Array Length After Pair Removals
// Java reference: src/test/java/g2801_2900/s2856_minimum_array_length_after_pair_removals/SolutionTest.java

use leetcode_in_rust::s2856::minimum_array_length_after_pair_removals::Solution;

#[test]
fn test_min_length_after_removals() {
    assert_eq!(Solution::min_length_after_removals(vec![1, 3, 4, 9]), 0);
}

#[test]
fn test_min_length_after_removals2() {
    assert_eq!(Solution::min_length_after_removals(vec![2, 3, 6, 9]), 0);
}

#[test]
fn test_min_length_after_removals3() {
    assert_eq!(Solution::min_length_after_removals(vec![1, 1, 2]), 1);
}
