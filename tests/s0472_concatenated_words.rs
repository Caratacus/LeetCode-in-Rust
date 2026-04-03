// Tests for Problem 0472: Concatenated Words
// Java reference: src/test/java/g0401_0500/s0472_concatenated_words/SolutionTest.java

use leetcode_in_rust::s0472::concatenated_words::Solution;

#[test]
fn test_find_all_concatenated_words_in_a_dict() {
    let words = vec![
        "cat".to_string(),
        "cats".to_string(),
        "catsdogcats".to_string(),
        "dog".to_string(),
        "dogcatsdog".to_string(),
        "hippopotamuses".to_string(),
        "rat".to_string(),
        "ratcatdogcat".to_string(),
    ];
    let result = Solution::find_all_concatenated_words_in_a_dict(words);
    // Expected: ["dogcatsdog", "catsdogcats", "ratcatdogcat"]
    assert!(result.contains(&"dogcatsdog".to_string()));
    assert!(result.contains(&"catsdogcats".to_string()));
    assert!(result.contains(&"ratcatdogcat".to_string()));
    assert_eq!(result.len(), 3);
}

#[test]
fn test_find_all_concatenated_words_in_a_dict2() {
    let words = vec!["cat".to_string(), "dog".to_string(), "catdog".to_string()];
    let result = Solution::find_all_concatenated_words_in_a_dict(words);
    assert_eq!(result, vec!["catdog".to_string()]);
}
