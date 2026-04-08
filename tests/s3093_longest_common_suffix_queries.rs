// Tests for Problem 3093: Longest Common Suffix Queries
// Java reference: src/test/java/g3001_3100/s3093_longest_common_suffix_queries/SolutionTest.java

use leetcode_in_rust::s3093::longest_common_suffix_queries::Solution;

#[test]
fn test_string_indices() {
    assert_eq!(
        Solution::string_indices(
            vec!["abcd".to_string(), "bcd".to_string(), "xbcd".to_string()],
            vec!["cd".to_string(), "bcd".to_string(), "xyz".to_string()]
        ),
        vec![1, 1, 1]
    );
}

#[test]
fn test_string_indices2() {
    assert_eq!(
        Solution::string_indices(
            vec![
                "abcdefgh".to_string(),
                "poiuygh".to_string(),
                "ghghgh".to_string()
            ],
            vec!["gh".to_string(), "acbfgh".to_string(), "acbfegh".to_string()]
        ),
        vec![2, 0, 2]
    );
}
