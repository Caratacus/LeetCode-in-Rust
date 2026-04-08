// Tests for Problem 3190: Find Minimum Operations to Make All Elements Divisible by Three
// Java reference: src/test/java/g3101_3200/s3190_find_minimum_operations_to_make_all_elements_divisible_by_three/SolutionTest.java

use leetcode_in_rust::s3190::find_minimum_operations_to_make_all_elements_divisible_by_three::Solution;

#[test]
fn test_minimum_operations() {
    assert_eq!(Solution::minimum_operations(vec![1, 2, 3, 4]), 3);
}

#[test]
fn test_minimum_operations2() {
    assert_eq!(Solution::minimum_operations(vec![3, 6, 9]), 0);
}
