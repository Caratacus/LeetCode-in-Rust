// Tests for Problem 2562: Find the Array Concatenation Value
// Java reference: src/test/java/g2501_2600/s2562_find_the_array_concatenation_value/SolutionTest.java

use leetcode_in_rust::s2562::find_the_array_concatenation_value::Solution;

#[test]
fn test_find_the_array_conc_val() {
    assert_eq!(Solution::find_the_array_conc_val(vec![7, 52, 2, 4]), 596);
}

#[test]
fn test_find_the_array_conc_val2() {
    assert_eq!(Solution::find_the_array_conc_val(vec![5, 14, 13, 8, 12]), 673);
}
