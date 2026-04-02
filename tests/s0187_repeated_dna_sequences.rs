// Tests for Problem 0187: Repeated DNA Sequences
// Java reference: src/test/java/g0181_0200/s0187_repeated_dna_sequences/SolutionTest.java

use leetcode_in_rust::s0187::repeated_dna_sequences::Solution;

#[test]
fn test_find_repeated_dna_sequences() {
    let result = Solution::find_repeated_dna_sequences(String::from("AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT"));
    assert!(result.contains(&String::from("AAAAACCCCC")));
    assert!(result.contains(&String::from("CCCCCAAAAA")));
}
