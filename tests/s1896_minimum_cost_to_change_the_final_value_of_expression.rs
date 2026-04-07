// Tests for Problem 1896: Minimum Cost to Change the Final Value of Expression
// Java reference: src/test/java/g1801_1900/s1896_minimum_cost_to_change_the_final_value_of_expression/SolutionTest.java

use leetcode_in_rust::s1896::minimum_cost_to_change_the_final_value_of_expression::Solution;

#[test]
fn test_min_operations_to_flip() {
    assert_eq!(Solution::min_operations_to_flip("1&(0|1)".to_string()), 1);
}

#[test]
fn test_min_operations_to_flip2() {
    assert_eq!(Solution::min_operations_to_flip("(0&0)&(0&0&0)".to_string()), 3);
}

#[test]
fn test_min_operations_to_flip3() {
    assert_eq!(Solution::min_operations_to_flip("(0|(1|0&1))".to_string()), 1);
}
