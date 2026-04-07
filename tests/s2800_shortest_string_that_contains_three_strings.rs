// Tests for Problem 2800: Shortest String That Contains Three Strings
// Java reference: src/test/java/g2701_2800/s2800_shortest_string_that_contains_three_strings/SolutionTest.java

use leetcode_in_rust::s2800::shortest_string_that_contains_three_strings::Solution;

#[test]
fn test_minimum_string() {
    assert_eq!(
        Solution::minimum_string("abc".to_string(), "bca".to_string(), "aaa".to_string()),
        "aaabca"
    );
}

#[test]
fn test_minimum_string2() {
    assert_eq!(
        Solution::minimum_string("ab".to_string(), "ba".to_string(), "aba".to_string()),
        "aba"
    );
}
