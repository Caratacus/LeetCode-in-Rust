// Tests for Problem 0071: Simplify Path
// Java reference: src/test/java/g0001_0100/s0071_simplify_path/SolutionTest.java

use leetcode_in_rust::s0071::simplify_path::Solution;

#[test]
fn test_simplify_path() {
    assert_eq!(Solution::simplify_path("/home/".to_string()), "/home");
}

#[test]
fn test_simplify_path2() {
    assert_eq!(Solution::simplify_path("/../".to_string()), "/");
}

#[test]
fn test_simplify_path3() {
    assert_eq!(Solution::simplify_path("/home//foo/".to_string()), "/home/foo");
}
