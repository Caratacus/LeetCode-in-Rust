// Tests for Problem 3228: Maximum Number of Operations to Move Ones to The End
// Java reference: src/test/java/g3201_3300/s3228_maximum_number_of_operations_to_move_ones_to_the_end/SolutionTest.java

use leetcode_in_rust::s3228::maximum_number_of_operations_to_move_ones_to_the_end::Solution;

#[test]
fn test_max_operations() {
    assert_eq!(Solution::max_operations("1001101".to_string()), 4);
}

#[test]
fn test_max_operations2() {
    assert_eq!(Solution::max_operations("00111".to_string()), 0);
}
