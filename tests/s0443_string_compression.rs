// Tests for Problem 0443: String Compression
// Java reference: src/test/java/g0401_0500/s0443_string_compression/SolutionTest.java

use leetcode_in_rust::s0443::string_compression::Solution;

#[test]
fn test_compress() {
    assert_eq!(Solution::compress(vec!['a', 'a', 'b', 'b', 'c', 'c', 'c']), 6);
}

#[test]
fn test_compress2() {
    assert_eq!(Solution::compress(vec!['a']), 1);
}

#[test]
fn test_compress3() {
    assert_eq!(
        Solution::compress(vec![
            'a', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b'
        ]),
        4
    );
}
