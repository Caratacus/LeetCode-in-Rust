// Tests for Problem 0474: Ones and Zeroes
// Java reference: src/test/java/g0401_0500/s0474_ones_and_zeroes/SolutionTest.java

use leetcode_in_rust::s0474::ones_and_zeroes::Solution;

#[test]
fn test_find_max_form() {
    assert_eq!(
        Solution::find_max_form(
            vec!["10".to_string(), "0001".to_string(), "111001".to_string(), "1".to_string(), "0".to_string()],
            5,
            3
        ),
        4
    );
}

#[test]
fn test_find_max_form2() {
    assert_eq!(
        Solution::find_max_form(vec!["10".to_string(), "0".to_string(), "1".to_string()], 1, 1),
        2
    );
}
