// Tests for Problem 0049: Group Anagrams
// Java reference: src/test/java/g0001_0100/s0049_group_anagrams/SolutionTest.java

use leetcode_in_rust::s0049::group_anagrams::Solution;

#[test]
fn test_group_anagrams() {
    let result = Solution::group_anagrams(vec![
        "eat".to_string(), "tea".to_string(), "tan".to_string(),
        "ate".to_string(), "nat".to_string(), "bat".to_string(),
    ]);
    // Result should have 3 groups
    assert_eq!(result.len(), 3);
}

#[test]
fn test_group_anagrams2() {
    let result = Solution::group_anagrams(vec!["".to_string()]);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], vec![""]);
}

#[test]
fn test_group_anagrams3() {
    let result = Solution::group_anagrams(vec!["a".to_string()]);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], vec!["a"]);
}
