// Tests for Problem 2154: Keep Multiplying Found Values by Two
// Java reference: src/test/java/g2101_2200/s2154_keep_multiplying_found_values_by_two/SolutionTest.java

use leetcode_in_rust::s2154::keep_multiplying_found_values_by_two::Solution;

#[test]
fn test_find_final_value() {
    assert_eq!(Solution::find_final_value(vec![5, 3, 6, 1, 12], 3), 24);
}

#[test]
fn test_find_final_value2() {
    assert_eq!(Solution::find_final_value(vec![2, 7, 9], 4), 4);
}
