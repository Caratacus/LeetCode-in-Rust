// Tests for Problem 1408: String Matching in an Array
// Java reference: src/test/java/g1401_1500/s1408_string_matching_in_an_array/SolutionTest.java

use leetcode_in_rust::s1408::string_matching_in_an_array::Solution;

#[test]
fn test_string_matching() {
    let result = Solution::string_matching(vec![
        "mass".to_string(),
        "as".to_string(),
        "hero".to_string(),
        "superhero".to_string(),
    ]);
    assert!(result.contains(&"as".to_string()));
    assert!(result.contains(&"hero".to_string()));
}

#[test]
fn test_string_matching2() {
    let result = Solution::string_matching(vec![
        "leetcode".to_string(),
        "et".to_string(),
        "code".to_string(),
    ]);
    assert!(result.contains(&"et".to_string()));
    assert!(result.contains(&"code".to_string()));
}

#[test]
fn test_string_matching3() {
    let result = Solution::string_matching(vec![
        "blue".to_string(),
        "green".to_string(),
        "bu".to_string(),
    ]);
    assert!(result.is_empty());
}
