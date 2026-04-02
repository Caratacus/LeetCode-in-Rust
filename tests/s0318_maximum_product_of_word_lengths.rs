// Tests for Problem 0318: Maximum Product of Word Lengths
// Java reference: src/test/java/g0301_0400/s0318_maximum_product_of_word_lengths/SolutionTest.java

use leetcode_in_rust::s0318::maximum_product_of_word_lengths::Solution;

#[test]
fn test_max_product() {
    assert_eq!(
        Solution::max_product(vec!["abcw".to_string(), "baz".to_string(), "foo".to_string(), "bar".to_string(), "xtfn".to_string(), "abcdef".to_string()]),
        16
    );
}

#[test]
fn test_max_product2() {
    assert_eq!(
        Solution::max_product(vec!["a".to_string(), "ab".to_string(), "abc".to_string(), "d".to_string(), "cd".to_string(), "bcd".to_string(), "abcd".to_string()]),
        4
    );
}
