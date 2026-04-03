// Tests for Problem 1763: Longest Nice Substring
// Java reference: src/test/java/g1701_1800/s1763_longest_nice_substring/SolutionTest.java

use leetcode_in_rust::s1763::longest_nice_substring::Solution;

#[test]
fn test_longest_nice_substring() {
    assert_eq!(Solution::longest_nice_substring("YazaAay"), "aAa");
}

#[test]
fn test_longest_nice_substring2() {
    assert_eq!(Solution::longest_nice_substring("Bb"), "Bb");
}

#[test]
fn test_longest_nice_substring3() {
    assert_eq!(Solution::longest_nice_substring("c"), "");
}
