// Tests for Problem 1313: Decompress Run-Length Encoded List
// Java reference: src/test/java/g1301_1400/s1313_decompress_run_length_encoded_list/SolutionTest.java

use leetcode_in_rust::s1313::decompress_run_length_encoded_list::Solution;

#[test]
fn test_decompress_rl_elist() {
    assert_eq!(Solution::decompress_rl_elist(vec![1, 2, 3, 4]), vec![2, 4, 4, 4]);
}

#[test]
fn test_decompress_rl_elist2() {
    assert_eq!(Solution::decompress_rl_elist(vec![1, 1, 2, 3]), vec![1, 3, 3]);
}
