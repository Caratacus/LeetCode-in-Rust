// Tests for Problem 0906: Super Palindromes
// Java reference: src/test/java/g0901_1000/s0906_super_palindromes/SolutionTest.java

use leetcode_in_rust::s0906::super_palindromes::Solution;

#[test]
fn test_superpalindromes_in_range() {
    let result = Solution::superpalindromes_in_range("4".to_string(), "1000".to_string());
    assert_eq!(result, 4);
}

#[test]
fn test_superpalindromes_in_range2() {
    let result = Solution::superpalindromes_in_range("1".to_string(), "2".to_string());
    assert_eq!(result, 1);
}
