// Tests for Problem 1531: String Compression II
// Java reference: src/test/java/g1501_1600/s1531_string_compression_ii/SolutionTest.java

use leetcode_in_rust::s1531::string_compression_ii::Solution;

#[test]
fn test_get_length_of_optimal_compression() {
    assert_eq!(Solution::get_length_of_optimal_compression("aaabcccd".to_string(), 2), 4);
}

#[test]
fn test_get_length_of_optimal_compression2() {
    assert_eq!(Solution::get_length_of_optimal_compression("aabbaa".to_string(), 2), 2);
}

#[test]
fn test_get_length_of_optimal_compression3() {
    assert_eq!(Solution::get_length_of_optimal_compression("aaaaaaaaaaa".to_string(), 0), 3);
}
