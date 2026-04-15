// Tests for Problem 3443: Maximum Manhattan Distance After K Changes
// Java reference: src/test/java/g3401_3500/s3443_maximum_manhattan_distance_after_k_changes/SolutionTest.java

use leetcode_in_rust::s3443::maximum_manhattan_distance_after_k_changes::Solution;

#[test]
fn test_max_distance() {
    assert_eq!(Solution::max_distance("NWSE".to_string(), 1), 3);
}

#[test]
fn test_max_distance2() {
    assert_eq!(Solution::max_distance("NSWWEW".to_string(), 3), 6);
}
