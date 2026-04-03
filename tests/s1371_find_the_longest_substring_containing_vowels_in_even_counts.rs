// Tests for Problem 1371: Find the Longest Substring Containing Vowels in Even Counts
// Java reference: src/test/java/g1301_1400/s1371_find_the_longest_substring_containing_vowels_in_even_counts/SolutionTest.java

use leetcode_in_rust::s1371::find_the_longest_substring_containing_vowels_in_even_counts::Solution;

#[test]
fn test_find_the_longest_substring() {
    assert_eq!(Solution::find_the_longest_substring("eleetminicoworoep".to_string()), 13);
}

#[test]
fn test_find_the_longest_substring2() {
    assert_eq!(Solution::find_the_longest_substring("leetcodeisgreat".to_string()), 5);
}

#[test]
fn test_find_the_longest_substring3() {
    assert_eq!(Solution::find_the_longest_substring("bcbcbc".to_string()), 6);
}
