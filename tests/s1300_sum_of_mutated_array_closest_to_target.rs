// Tests for Problem 1300: Sum of Mutated Array Closest to Target
// Java reference: src/test/java/g1201_1300/s1300_sum_of_mutated_array_closest_to_target/SolutionTest.java

use leetcode_in_rust::s1300::sum_of_mutated_array_closest_to_target::Solution;

#[test]
fn test_find_best_value() {
    let result = Solution::find_best_value(vec![4, 9, 3], 10);
    assert_eq!(result, 3);
}

#[test]
fn test_find_best_value2() {
    let result = Solution::find_best_value(vec![2, 3, 5], 10);
    assert_eq!(result, 5);
}
