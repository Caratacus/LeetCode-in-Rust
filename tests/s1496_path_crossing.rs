// Tests for Problem 1496: Path Crossing
// Java reference: src/test/java/g1401_1500/s1496_path_crossing/SolutionTest.java

use leetcode_in_rust::s1496::path_crossing::Solution;

#[test]
fn test_is_path_crossing() {
    assert_eq!(Solution::is_path_crossing("NES".to_string()), false);
}

#[test]
fn test_is_path_crossing2() {
    assert_eq!(Solution::is_path_crossing("NESWW".to_string()), true);
}
