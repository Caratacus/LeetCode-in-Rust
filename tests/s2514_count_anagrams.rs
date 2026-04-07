// Tests for Problem 2514: Count Anagrams
// Java reference: src/test/java/g2401_2500/s2514_count_anagrams/SolutionTest.java

use leetcode_in_rust::s2514::count_anagrams::Solution;

#[test]
fn test_count_anagrams() {
    assert_eq!(Solution::count_anagrams("too hot".to_string()), 18);
}

#[test]
fn test_count_anagrams2() {
    assert_eq!(Solution::count_anagrams("aa".to_string()), 1);
}
