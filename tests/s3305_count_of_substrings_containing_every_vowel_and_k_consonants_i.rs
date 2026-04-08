// Tests for Problem 3305: Count of Substrings Containing Every Vowel and K Consonants I
// Java reference: src/test/java/g3301_3400/s3305_count_of_substrings_containing_every_vowel_and_k_consonants_i/SolutionTest.java

use leetcode_in_rust::s3305::count_of_substrings_containing_every_vowel_and_k_consonants_i::Solution;

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
