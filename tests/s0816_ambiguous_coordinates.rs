// Tests for Problem 0816: Ambiguous Coordinates
// Java reference: src/test/java/g0701_0800/s0816_ambiguous_coordinates/SolutionTest.java

use leetcode_in_rust::s0816::ambiguous_coordinates::Solution;

#[test]
fn test_ambiguous_coordinates() {
    let mut result = Solution::ambiguous_coordinates("(123)".to_string());
    result.sort();
    assert_eq!(result, vec!["(1, 23)", "(1, 2.3)", "(12, 3)", "(1.2, 3)"]);
}

