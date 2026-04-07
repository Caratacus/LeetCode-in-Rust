// Tests for Problem 2011: Final Value of Variable After Performing Operations
// Java reference: src/test/java/g2001_2100/s2011_final_value_of_variable_after_performing_operations/SolutionTest.java

use leetcode_in_rust::s2011::final_value_of_variable_after_performing_operations::Solution;

#[test]
fn test_final_value_after_operations() {
    assert_eq!(
        Solution::final_value_after_operations(vec![
            String::from("--X"),
            String::from("X++"),
            String::from("X++")
        ]),
        1
    );
}

#[test]
fn test_final_value_after_operations2() {
    assert_eq!(
        Solution::final_value_after_operations(vec![
            String::from("++X"),
            String::from("++X"),
            String::from("X++")
        ]),
        3
    );
}

#[test]
fn test_final_value_after_operations3() {
    assert_eq!(
        Solution::final_value_after_operations(vec![
            String::from("X++"),
            String::from("++X"),
            String::from("--X"),
            String::from("X--")
        ]),
        0
    );
}
