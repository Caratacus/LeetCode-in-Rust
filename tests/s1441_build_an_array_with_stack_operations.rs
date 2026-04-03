// Tests for Problem 1441: Build an Array With Stack Operations
// Java reference: src/test/java/g1401_1500/s1441_build_an_array_with_stack_operations/SolutionTest.java

use leetcode_in_rust::s1441::build_an_array_with_stack_operations::Solution;

#[test]
fn test_build_array() {
    let expected = vec!["Push".to_string(), "Push".to_string(), "Pop".to_string(), "Push".to_string()];
    assert_eq!(Solution::build_array(vec![1, 3], 3), expected);
}

#[test]
fn test_build_array2() {
    let expected = vec!["Push".to_string(), "Push".to_string(), "Push".to_string()];
    assert_eq!(Solution::build_array(vec![1, 2, 3], 3), expected);
}

#[test]
fn test_build_array3() {
    let expected = vec!["Push".to_string(), "Push".to_string()];
    assert_eq!(Solution::build_array(vec![1, 2], 4), expected);
}
