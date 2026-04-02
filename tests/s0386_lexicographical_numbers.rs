// Tests for Problem 0386: Lexicographical Numbers
// Java reference: src/test/java/g0301_0400/s0386_lexicographical_numbers/SolutionTest.java

use leetcode_in_rust::s0386::lexicographical_numbers::Solution;

#[test]
fn test_lexical_order() {
    assert_eq!(
        Solution::lexical_order(13),
        vec![1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9]
    );
}

#[test]
fn test_lexical_order2() {
    assert_eq!(Solution::lexical_order(2), vec![1, 2]);
}
