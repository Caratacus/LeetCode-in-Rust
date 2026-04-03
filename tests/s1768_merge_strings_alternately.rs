// Tests for Problem 1768: Merge Strings Alternately
// Java reference: src/test/java/g1701_1800/s1768_merge_strings_alternately/SolutionTest.java

use leetcode_in_rust::s1768::merge_strings_alternately::Solution;

#[test]
fn test_merge_alternately() {
    assert_eq!(Solution::merge_alternately("abc", "pqr"), "apbqcr");
}

#[test]
fn test_merge_alternately2() {
    assert_eq!(Solution::merge_alternately("ab", "pqrs"), "apbqrs");
}

#[test]
fn test_merge_alternately3() {
    assert_eq!(Solution::merge_alternately("abcd", "pq"), "apbqcd");
}
