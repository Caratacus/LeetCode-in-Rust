// Tests for Problem 1190: Reverse Substrings Between Each Pair of Parentheses
// Java reference: src/test/java/g1101_1200/s1190_reverse_substrings_between_each_pair_of_parentheses/SolutionTest.java

use leetcode_in_rust::s1190::reverse_substrings_between_each_pair_of_parentheses::Solution;

#[test]
fn test_reverse_parentheses() {
    assert_eq!(Solution::reverse_parentheses("(abcd)".to_string()), "dcba");
}

#[test]
fn test_reverse_parentheses2() {
    assert_eq!(Solution::reverse_parentheses("(u(love)i)".to_string()), "iloveu");
}

#[test]
fn test_reverse_parentheses3() {
    assert_eq!(
        Solution::reverse_parentheses("(ed(et(oc))el)".to_string()),
        "leetcode"
    );
}
