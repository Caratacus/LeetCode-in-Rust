// Tests for Problem 1365: How Many Numbers Are Smaller Than the Current Number
// Java reference: src/test/java/g1301_1400/s1365_how_many_numbers_are_smaller_than_the_current_number/SolutionTest.java

use leetcode_in_rust::s1365::how_many_numbers_are_smaller_than_the_current_number::Solution;

#[test]
fn test_smaller_numbers_than_current() {
    assert_eq!(
        Solution::smaller_numbers_than_current(vec![8, 1, 2, 2, 3]),
        vec![4, 0, 1, 1, 3]
    );
}

#[test]
fn test_smaller_numbers_than_current2() {
    assert_eq!(
        Solution::smaller_numbers_than_current(vec![6, 5, 4, 8]),
        vec![2, 1, 0, 3]
    );
}

#[test]
fn test_smaller_numbers_than_current3() {
    assert_eq!(
        Solution::smaller_numbers_than_current(vec![7, 7, 7, 7]),
        vec![0, 0, 0, 0]
    );
}
