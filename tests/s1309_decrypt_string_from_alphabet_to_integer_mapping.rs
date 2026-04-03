// Tests for Problem 1309: Decrypt String from Alphabet to Integer Mapping
// Java reference: src/test/java/g1301_1400/s1309_decrypt_string_from_alphabet_to_integer_mapping/SolutionTest.java

use leetcode_in_rust::s1309::decrypt_string_from_alphabet_to_integer_mapping::Solution;

#[test]
fn test_freq_alphabets() {
    assert_eq!(Solution::freq_alphabets("10#11#12".to_string()), "jkab");
}

#[test]
fn test_freq_alphabets2() {
    assert_eq!(Solution::freq_alphabets("1326#".to_string()), "acz");
}
