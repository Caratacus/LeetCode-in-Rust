// Tests for Problem 2606: Find the Substring With Maximum Cost
// Java reference: src/test/java/g2601_2700/s2606_find_the_substring_with_maximum_cost/SolutionTest.java

use leetcode_in_rust::s2606::find_the_substring_with_maximum_cost::Solution;

#[test]
fn test_maximum_cost_substring() {
    assert_eq!(
        Solution::maximum_cost_substring("adaa".to_string(), "d".to_string(), vec![-1000]),
        2
    );
}

#[test]
fn test_maximum_cost_substring2() {
    assert_eq!(
        Solution::maximum_cost_substring("abc".to_string(), "abc".to_string(), vec![-1, -1, -1]),
        0
    );
}
