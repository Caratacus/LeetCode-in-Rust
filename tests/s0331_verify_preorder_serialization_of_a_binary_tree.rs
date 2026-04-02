// Tests for Problem 0331: Verify Preorder Serialization of a Binary Tree
// Java reference: src/test/java/g0301_0400/s0331_verify_preorder_serialization_of_a_binary_tree/SolutionTest.java

use leetcode_in_rust::s0331::verify_preorder_serialization_of_a_binary_tree::Solution;

#[test]
fn test_is_valid_serialization() {
    assert_eq!(Solution::is_valid_serialization("9,3,4,#,#,1,#,#,2,#,6,#,#".to_string()), true);
}

#[test]
fn test_is_valid_serialization2() {
    assert_eq!(Solution::is_valid_serialization("1,#".to_string()), false);
}

#[test]
fn test_is_valid_serialization3() {
    assert_eq!(Solution::is_valid_serialization("9,#,#,1".to_string()), false);
}
