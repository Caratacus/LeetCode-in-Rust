// Tests for Problem 2559: Count Vowel Strings in Ranges
// Java reference: src/test/java/g2501_2600/s2559_count_vowel_strings_in_ranges/SolutionTest.java

use leetcode_in_rust::s2559::count_vowel_strings_in_ranges::Solution;

#[test]
fn test_vowel_strings() {
    assert_eq!(
        Solution::vowel_strings(
            vec!["aba".to_string(), "bcb".to_string(), "ece".to_string(), "aa".to_string(), "e".to_string()],
            vec![vec![0, 2], vec![1, 4], vec![1, 1]]
        ),
        vec![2, 3, 0]
    );
}

#[test]
fn test_vowel_strings2() {
    assert_eq!(
        Solution::vowel_strings(
            vec!["a".to_string(), "e".to_string(), "i".to_string()],
            vec![vec![0, 2], vec![0, 1], vec![2, 2]]
        ),
        vec![3, 2, 1]
    );
}
