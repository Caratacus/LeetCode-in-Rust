// Tests for Problem 0115: Distinct Subsequences
// Java reference: src/test/java/g0101_0200/s0115_distinct_subsequences/SolutionTest.java

use leetcode_in_rust::s0115::distinct_subsequences::Solution;

#[test]
fn test_num_distinct() {
    assert_eq!(Solution::num_distinct(String::from("rabbbit"), String::from("rabbit")), 3);
}

#[test]
fn test_num_distinct2() {
    assert_eq!(Solution::num_distinct(String::from("babgbag"), String::from("bag")), 5);
}

#[test]
fn test_num_distinct3() {
    assert_eq!(Solution::num_distinct(String::from(""), String::from("a")), 0);
}
