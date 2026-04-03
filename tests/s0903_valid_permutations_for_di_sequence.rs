// Tests for Problem 0903: Valid Permutations for DI Sequence
// Java reference: src/test/java/g0901_1000/s0903_valid_permutations_for_di_sequence/SolutionTest.java

use leetcode_in_rust::s0903::valid_permutations_for_di_sequence::Solution;

#[test]
fn test_num_perms_di_sequence() {
    let result = Solution::num_perms_di_sequence("DID".to_string());
    assert_eq!(result, 5);
}

#[test]
fn test_num_perms_di_sequence2() {
    let result = Solution::num_perms_di_sequence("D".to_string());
    assert_eq!(result, 1);
}
