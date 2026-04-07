// Tests for Problem 1781: Sum of Beauty of All Substrings
// Java reference: src/test/java/g1701_1800/s1781_sum_of_beauty_of_all_substrings/SolutionTest.java

use leetcode_in_rust::s1781::sum_of_beauty_of_all_substrings::Solution;

#[test]
fn test_beauty_sum() {
    assert_eq!(Solution::beauty_sum("aabcb".to_string()), 5);
}

#[test]
fn test_beauty_sum2() {
    assert_eq!(Solution::beauty_sum("aabcbaa".to_string()), 17);
}
