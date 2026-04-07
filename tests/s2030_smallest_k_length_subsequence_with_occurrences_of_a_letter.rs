// Tests for Problem 2030: Smallest K-Length Subsequence With Occurrences of a Letter
// Java reference: src/test/java/g2001_2100/s2030_smallest_k_length_subsequence_with_occurrences_of_a_letter/SolutionTest.java

use leetcode_in_rust::s2030::smallest_k_length_subsequence_with_occurrences_of_a_letter::Solution;

#[test]
fn test_smallest_subsequence() {
    assert_eq!(
        Solution::smallest_subsequence("leet".to_string(), 3, 'e', 1),
        "eet".to_string()
    );
}

#[test]
fn test_smallest_subsequence2() {
    assert_eq!(
        Solution::smallest_subsequence("leetcode".to_string(), 4, 'e', 2),
        "ecde".to_string()
    );
}

#[test]
fn test_smallest_subsequence3() {
    assert_eq!(
        Solution::smallest_subsequence("bb".to_string(), 2, 'b', 2),
        "bb".to_string()
    );
}
