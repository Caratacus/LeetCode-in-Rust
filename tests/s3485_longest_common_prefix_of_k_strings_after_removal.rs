// Tests for Problem 3485: Longest Common Prefix of K Strings After Removal
// Java reference: src/test/java/g3401_3500/s3485_longest_common_prefix_of_k_strings_after_removal/SolutionTest.java

use leetcode_in_rust::s3485::longest_common_prefix_of_k_strings_after_removal::Solution;

#[test]
fn test_longest_common_prefix() {
    assert_eq!(
        Solution::longest_common_prefix(vec!["jump".to_string(), "run".to_string(), "run".to_string(), "jump".to_string(), "run".to_string()], 2),
        vec![3, 4, 4, 3, 4]
    );
}

#[test]
fn test_longest_common_prefix2() {
    assert_eq!(
        Solution::longest_common_prefix(vec!["dog".to_string(), "racer".to_string(), "car".to_string()], 2),
        vec![0, 0, 0]
    );
}

#[test]
fn test_longest_common_prefix3() {
    assert_eq!(
        Solution::longest_common_prefix(vec!["cdbff".to_string()], 1),
        vec![0]
    );
}
