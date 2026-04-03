// Tests for Problem 0639: Decode Ways II
// Java reference: src/test/java/g0601_0700/s0639_decode_ways_ii/SolutionTest.java

use leetcode_in_rust::s0639::decode_ways_ii::Solution;

#[test]
fn test_num_decodings() {
    assert_eq!(Solution::num_decodings("*".to_string()), 9);
}

#[test]
fn test_num_decodings2() {
    assert_eq!(Solution::num_decodings("1*".to_string()), 18);
}

#[test]
fn test_num_decodings3() {
    assert_eq!(Solution::num_decodings("2*".to_string()), 15);
}

#[test]
fn test_num_decodings4() {
    assert_eq!(Solution::num_decodings("134*92*0*9*".to_string()), 3600);
}
