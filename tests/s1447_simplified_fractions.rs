// Tests for Problem 1447: Simplified Fractions
// Java reference: src/test/java/g1401_1500/s1447_simplified_fractions/SolutionTest.java

use leetcode_in_rust::s1447::simplified_fractions::Solution;

#[test]
fn test_simplified_fractions() {
    let result = Solution::simplified_fractions(2);
    assert_eq!(result, vec!["1/2".to_string()]);
}

#[test]
fn test_simplified_fractions2() {
    let result = Solution::simplified_fractions(4);
    let expected = vec!["1/2".to_string(), "1/3".to_string(), "2/3".to_string(), "1/4".to_string(), "3/4".to_string()];
    assert_eq!(result, expected);
}
