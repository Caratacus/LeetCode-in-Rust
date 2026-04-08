// Tests for Problem 3306: Count of Substrings Containing Every Vowel and K Consonants II
// Java reference: src/test/java/g3301_3400/s3306_count_of_substrings_containing_every_vowel_and_k_consonants_ii/SolutionTest.java

use leetcode_in_rust::s3306::count_of_substrings_containing_every_vowel_and_k_consonants_ii::Solution;

#[test]
fn test_count_of_substrings() {
    assert_eq!(Solution::count_of_substrings("aeioqq".to_string(), 1), 0);
}

#[test]
fn test_count_of_substrings2() {
    assert_eq!(Solution::count_of_substrings("aeiou".to_string(), 0), 1);
}

#[test]
fn test_count_of_substrings3() {
    assert_eq!(Solution::count_of_substrings("ieaouqqieaouqq".to_string(), 1), 3);
}
