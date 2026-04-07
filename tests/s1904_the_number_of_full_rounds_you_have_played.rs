// Tests for Problem 1904: The Number of Full Rounds You Have Played
// Java reference: src/test/java/g1901_2000/s1904_the_number_of_full_rounds_you_have_played/SolutionTest.java

use leetcode_in_rust::s1904::the_number_of_full_rounds_you_have_played::Solution;

#[test]
fn test_number_of_rounds() {
    assert_eq!(
        Solution::number_of_rounds("09:31".to_string(), "10:14".to_string()),
        1
    );
}

#[test]
fn test_number_of_rounds2() {
    assert_eq!(
        Solution::number_of_rounds("21:30".to_string(), "03:00".to_string()),
        22
    );
}

#[test]
fn test_number_of_rounds3() {
    assert_eq!(
        Solution::number_of_rounds("21:30".to_string(), "21:30".to_string()),
        0
    );
}
