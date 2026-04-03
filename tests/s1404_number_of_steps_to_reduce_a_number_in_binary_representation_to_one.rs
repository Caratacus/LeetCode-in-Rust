// Tests for Problem 1404: Number of Steps to Reduce a Number in Binary Representation to One
// Java reference: src/test/java/g1301_1400/s1404_number_of_steps_to_reduce_a_number_in_binary_representation_to_one/SolutionTest.java

use leetcode_in_rust::s1404::number_of_steps_to_reduce_a_number_in_binary_representation_to_one::Solution;

#[test]
fn test_num_steps() {
    assert_eq!(Solution::num_steps("1101".to_string()), 6);
}

#[test]
fn test_num_steps2() {
    assert_eq!(Solution::num_steps("10".to_string()), 1);
}

#[test]
fn test_num_steps3() {
    assert_eq!(Solution::num_steps("1".to_string()), 0);
}
