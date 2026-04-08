// Tests for Problem 3163: String Compression III
// Java reference: src/test/java/g3101_3200/s3163_string_compression_iii/SolutionTest.java

use leetcode_in_rust::s3163::string_compression_iii::Solution;
#[test]
fn test_compressed_string() {
    assert_eq!(Solution::compressed_string(String::from("abcde")), String::from("1a1b1c1d1e"));
}
#[test]
fn test_compressed_string2() {
    assert_eq!(Solution::compressed_string(String::from("aaaaaaaaaaaaaabb")), String::from("9a5a2b"));
}
