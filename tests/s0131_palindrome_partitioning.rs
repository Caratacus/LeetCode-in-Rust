// Tests for Problem 0131: Palindrome Partitioning
// Java reference: src/test/java/g0121_0200/s0131_palindrome_partitioning/SolutionTest.java

use leetcode_in_rust::s0131::palindrome_partitioning::Solution;

#[test]
fn test_partition() {
    let result = Solution::partition(String::from("aab"));
    assert!(result.contains(&vec![String::from("a"), String::from("a"), String::from("b")]));
    assert!(result.contains(&vec![String::from("aa"), String::from("b")]));
}
