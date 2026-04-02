// Tests for Problem 0916: Word Subsets
// Java reference: src/test/java/g0901_1000/s0916_word_subsets/SolutionTest.java

use leetcode_in_rust::s0916::word_subsets::Solution;

fn sorted(vec: Vec<String>) -> Vec<String> {
    let mut v = vec;
    v.sort();
    v
}

#[test]
fn test_word_subsets() {
    let result = sorted(Solution::word_subsets(
        vec!["amazon".to_string(), "apple".to_string(), "facebook".to_string(), "google".to_string(), "leetcode".to_string()],
        vec!["e".to_string(), "o".to_string()]
    ));
    assert_eq!(result, vec!["facebook".to_string(), "google".to_string(), "leetcode".to_string()]);
}

#[test]
fn test_word_subsets2() {
    let result = sorted(Solution::word_subsets(
        vec!["amazon".to_string(), "apple".to_string(), "facebook".to_string(), "google".to_string(), "leetcode".to_string()],
        vec!["l".to_string(), "e".to_string()]
    ));
    assert_eq!(result, vec!["apple".to_string(), "google".to_string(), "leetcode".to_string()]);
}
