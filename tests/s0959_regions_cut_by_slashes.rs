// Tests for Problem 0959: Regions Cut by Slashes
// Java reference: src/test/java/g0901_1000/s0959_regions_cut_by_slashes/SolutionTest.java

use leetcode_in_rust::s0959::regions_cut_by_slashes::Solution;

#[test]
fn test_regions_by_slashes() {
    let result = Solution::regions_by_slashes(vec![" /".to_string(), "/ ".to_string()]);
    assert_eq!(result, 2);
}

#[test]
fn test_regions_by_slashes2() {
    let result = Solution::regions_by_slashes(vec![" /".to_string(), "  ".to_string()]);
    assert_eq!(result, 1);
}

#[test]
fn test_regions_by_slashes3() {
    let result = Solution::regions_by_slashes(vec!["/\\".to_string(), "\\/".to_string()]);
    assert_eq!(result, 5);
}
