// Tests for Problem 0946: Validate Stack Sequences
// Java reference: src/test/java/g0901_1000/s0946_validate_stack_sequences/SolutionTest.java

use leetcode_in_rust::s0946::validate_stack_sequences::Solution;

#[test]
fn test_validate_stack_sequences() {
    assert_eq!(
        Solution::validate_stack_sequences(vec![1, 2, 3, 4, 5], vec![4, 5, 3, 2, 1]),
        true
    );
}

#[test]
fn test_validate_stack_sequences2() {
    assert_eq!(
        Solution::validate_stack_sequences(vec![1, 2, 3, 4, 5], vec![4, 3, 5, 1, 2]),
        false
    );
}
