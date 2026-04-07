// Tests for Problem 1946: Largest Number After Mutating Substring
// Java reference: src/test/java/g1901_2000/s1946_largest_number_after_mutating_substring/SolutionTest.java

use leetcode_in_rust::s1946::largest_number_after_mutating_substring::Solution;

#[test]
fn test_maximum_number() {
    assert_eq!(
        Solution::maximum_number(String::from("132"), vec![9, 8, 5, 0, 3, 6, 4, 2, 6, 8]),
        String::from("832")
    );
}

#[test]
fn test_maximum_number2() {
    assert_eq!(
        Solution::maximum_number(String::from("021"), vec![9, 4, 3, 5, 7, 2, 1, 9, 0, 6]),
        String::from("934")
    );
}

#[test]
fn test_maximum_number3() {
    assert_eq!(
        Solution::maximum_number(String::from("5"), vec![1, 4, 7, 5, 3, 2, 5, 6, 9, 4]),
        String::from("5")
    );
}
