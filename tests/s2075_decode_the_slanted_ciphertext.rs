// Tests for Problem 2075: Decode the Slanted Ciphertext
// Java reference: src/test/java/g2001_2100/s2075_decode_the_slanted_ciphertext/SolutionTest.java

use leetcode_in_rust::s2075::decode_the_slanted_ciphertext::Solution;

#[test]
fn test_decode_ciphertext() {
    assert_eq!(
        Solution::decode_ciphertext("ch   ie   pr".to_string(), 3),
        "cipher"
    );
}

#[test]
fn test_decode_ciphertext2() {
    assert_eq!(
        Solution::decode_ciphertext("iveo    eed   l te   olc".to_string(), 4),
        "i love leetcode"
    );
}

#[test]
fn test_decode_ciphertext3() {
    assert_eq!(Solution::decode_ciphertext("coding".to_string(), 1), "coding");
}
