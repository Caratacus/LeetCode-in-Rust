// Tests for Problem 3403: Find the Lexicographically Largest String From the Box I
// Java reference: src/test/java/g3401_3500/s3403_find_the_lexicographically_largest_string_from_the_box_i/SolutionTest.java

use leetcode_in_rust::s3403::find_the_lexicographically_largest_string_from_the_box_i::Solution;

#[test]
fn test_answer_string() {
    assert_eq!(Solution::answer_string("dbca".to_string(), 2), "dbc");
}

#[test]
fn test_answer_string2() {
    assert_eq!(Solution::answer_string("gggg".to_string(), 4), "g");
}

#[test]
fn test_answer_string3() {
    assert_eq!(Solution::answer_string("a".to_string(), 1), "a");
}
