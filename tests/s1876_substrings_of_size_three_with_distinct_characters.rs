// Tests for Problem 1876: Substrings of Size Three with Distinct Characters
// Java reference: src/test/java/g1801_1900/s1876_substrings_of_size_three_with_distinct_characters/SolutionTest.java

use leetcode_in_rust::s1876::substrings_of_size_three_with_distinct_characters::Solution;

#[test]
fn test_count_good_substrings() {
    assert_eq!(Solution::count_good_substrings("xyzzaz".to_string()), 1);
}

#[test]
fn test_count_good_substrings2() {
    assert_eq!(Solution::count_good_substrings("aababcabc".to_string()), 4);
}
