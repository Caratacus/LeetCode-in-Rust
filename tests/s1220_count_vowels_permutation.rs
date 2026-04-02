// Tests for Problem 1220: Count Vowels Permutation
// Java reference: src/test/java/g1201_1300/s1220_count_vowels_permutation/SolutionTest.java

use leetcode_in_rust::s1220::count_vowels_permutation::Solution;

#[test]
fn test_count_vowel_permutation() {
    assert_eq!(Solution::count_vowel_permutation(1), 5);
}

#[test]
fn test_count_vowel_permutation2() {
    assert_eq!(Solution::count_vowel_permutation(2), 10);
}

#[test]
fn test_count_vowel_permutation3() {
    assert_eq!(Solution::count_vowel_permutation(5), 68);
}
