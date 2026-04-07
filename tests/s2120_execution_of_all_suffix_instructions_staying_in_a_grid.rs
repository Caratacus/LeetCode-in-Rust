// Tests for Problem 2120: Execution of All Suffix Instructions Staying in a Grid
// Java reference: src/test/java/g2101_2200/s2120_execution_of_all_suffix_instructions_staying_in_a_grid/SolutionTest.java

use leetcode_in_rust::s2120::execution_of_all_suffix_instructions_staying_in_a_grid::Solution;

#[test]
fn test_execute_instructions() {
    assert_eq!(
        Solution::execute_instructions(3, vec![0, 1], "RRDDLU".to_string()),
        vec![1, 5, 4, 3, 1, 0]
    );
}

#[test]
fn test_execute_instructions2() {
    assert_eq!(
        Solution::execute_instructions(2, vec![1, 1], "LURD".to_string()),
        vec![4, 1, 0, 0]
    );
}

#[test]
fn test_execute_instructions3() {
    assert_eq!(
        Solution::execute_instructions(1, vec![0, 0], "LRUD".to_string()),
        vec![0, 0, 0, 0]
    );
}
