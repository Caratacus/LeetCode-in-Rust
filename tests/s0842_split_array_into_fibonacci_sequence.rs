// Tests for Problem 0842: Split Array into Fibonacci Sequence
// Java reference: src/test/java/g0801_0900/s0842_split_array_into_fibonacci_sequence/SolutionTest.java

use leetcode_in_rust::s0842::split_array_into_fibonacci_sequence::Solution;

#[test]
fn test_split_into_fibonacci() {
    assert_eq!(
        Solution::split_into_fibonacci("1101111".to_string()),
        vec![11, 0, 11, 11]
    );
}

#[test]
fn test_split_into_fibonacci2() {
    assert_eq!(
        Solution::split_into_fibonacci("112358130".to_string()),
        vec![]
    );
}

#[test]
fn test_split_into_fibonacci3() {
    assert_eq!(Solution::split_into_fibonacci("0123".to_string()), vec![]);
}
