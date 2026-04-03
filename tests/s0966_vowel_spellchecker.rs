// Tests for Problem 0966: Vowel Spellchecker
// Java reference: src/test/java/g0901_1000/s0966_vowel_spellchecker/SolutionTest.java

use leetcode_in_rust::s0966::vowel_spellchecker::Solution;

#[test]
fn test_spellchecker() {
    let result = Solution::spellchecker(
        vec!["KiTe".to_string(), "kite".to_string(), "hare".to_string(), "Hare".to_string()],
        vec!["kite".to_string(), "KiTe".to_string(), "KiTe".to_string(), "Hare".to_string(), "HARE".to_string(), "Hear".to_string(), "hear".to_string(), "keti".to_string(), "keet".to_string(), "keto".to_string()],
    );
    assert_eq!(result, vec!["kite", "KiTe", "KiTe", "Hare", "hare", "Hare", "hare", "KiTe", "KiTe", "KiTe"]);
}

#[test]
fn test_spellchecker2() {
    let result = Solution::spellchecker(vec!["yellow".to_string()], vec!["YellOw".to_string()]);
    assert_eq!(result, vec!["yellow"]);
}
