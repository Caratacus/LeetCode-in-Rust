// Tests for Problem 2109: Adding Spaces to a String
// Java reference: src/test/java/g2101_2200/s2109_adding_spaces_to_a_string/SolutionTest.java

use leetcode_in_rust::s2109::adding_spaces_to_a_string::Solution;

#[test]
fn test_add_spaces() {
    assert_eq!(
        Solution::add_spaces("LeetcodeHelpsMeLearn".to_string(), vec![8, 13, 15]),
        "Leetcode Helps Me Learn"
    );
}

#[test]
fn test_add_spaces2() {
    assert_eq!(
        Solution::add_spaces("icodeinpython".to_string(), vec![1, 5, 7, 9]),
        "i code in py thon"
    );
}

#[test]
fn test_add_spaces3() {
    assert_eq!(
        Solution::add_spaces("spacing".to_string(), vec![0, 1, 2, 3, 4, 5, 6]),
        " s p a c i n g"
    );
}
