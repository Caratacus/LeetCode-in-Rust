// Tests for Problem 1233: Remove Sub-Folders from the Filesystem
// Java reference: src/test/java/g1201_1300/s1233_remove_sub_folders_from_the_filesystem/SolutionTest.java

use leetcode_in_rust::s1233::remove_sub_folders_from_the_filesystem::Solution;
use std::collections::HashSet;

#[test]
fn test_remove_subfolders() {
    let result = Solution::remove_subfolders(vec!["/a".to_string(), "/a/b".to_string(), "/c/d".to_string(), "/c/d/e".to_string(), "/c/f".to_string()]);
    let expected: HashSet<String> = ["/a", "/c/d", "/c/f"].iter().map(|s| s.to_string()).collect();
    let result_set: HashSet<String> = result.into_iter().collect();
    assert_eq!(result_set, expected);
}

#[test]
fn test_remove_subfolders2() {
    let result = Solution::remove_subfolders(vec!["/a".to_string(), "/a/b/c".to_string(), "/a/b/d".to_string()]);
    let expected: HashSet<String> = ["/a"].iter().map(|s| s.to_string()).collect();
    let result_set: HashSet<String> = result.into_iter().collect();
    assert_eq!(result_set, expected);
}

#[test]
fn test_remove_subfolders3() {
    let result = Solution::remove_subfolders(vec!["/a/b/c".to_string(), "/a/b/ca".to_string(), "/a/b/d".to_string()]);
    let expected: HashSet<String> = ["/a/b/c", "/a/b/ca", "/a/b/d"].iter().map(|s| s.to_string()).collect();
    let result_set: HashSet<String> = result.into_iter().collect();
    assert_eq!(result_set, expected);
}
