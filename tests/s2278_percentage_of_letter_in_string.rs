// Tests for Problem 2278: Percentage of Letter in String
// Java reference: src/test/java/g2201_2300/s2278_percentage_of_letter_in_string/SolutionTest.java

use leetcode_in_rust::s2278::percentage_of_letter_in_string::Solution;

#[test]
fn test_percentage_letter() {
    assert_eq!(Solution::percentage_letter(String::from("foobar"), 'o'), 33);
}

#[test]
fn test_percentage_letter2() {
    assert_eq!(Solution::percentage_letter(String::from("jjjj"), 'k'), 0);
}
