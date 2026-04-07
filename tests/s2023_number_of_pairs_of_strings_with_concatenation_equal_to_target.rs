// Tests for Problem 2023: Number of Pairs of Strings With Concatenation Equal to Target
// Java reference: src/test/java/g2001_2100/s2023_number_of_pairs_of_strings_with_concatenation_equal_to_target/SolutionTest.java

use leetcode_in_rust::s2023::number_of_pairs_of_strings_with_concatenation_equal_to_target::Solution;

#[test]
fn test_num_of_pairs() {
    assert_eq!(
        Solution::num_of_pairs(
            vec![
                String::from("777"),
                String::from("7"),
                String::from("77"),
                String::from("77")
            ],
            String::from("7777")
        ),
        4
    );
}

#[test]
fn test_num_of_pairs2() {
    assert_eq!(
        Solution::num_of_pairs(
            vec![
                String::from("123"),
                String::from("4"),
                String::from("12"),
                String::from("34")
            ],
            String::from("1234")
        ),
        2
    );
}

#[test]
fn test_num_of_pairs3() {
    assert_eq!(
        Solution::num_of_pairs(
            vec![String::from("1"), String::from("1"), String::from("1")],
            String::from("11")
        ),
        6
    );
}
