// Tests for Problem 1170: Compare Strings by Frequency of the Smallest Character
// Java reference: src/test/java/g1101_1200/s1170_compare_strings_by_frequency_of_the_smallest_character/SolutionTest.java

use leetcode_in_rust::s1170::compare_strings_by_frequency_of_the_smallest_character::Solution;

#[test]
fn test_num_smaller_by_frequency() {
    assert_eq!(
        Solution::num_smaller_by_frequency(vec!["cbd".to_string()], vec!["zaaaz".to_string()]),
        vec![1]
    );
}

#[test]
fn test_num_smaller_by_frequency2() {
    assert_eq!(
        Solution::num_smaller_by_frequency(
            vec!["bbb".to_string(), "cc".to_string()],
            vec![
                "a".to_string(),
                "aa".to_string(),
                "aaa".to_string(),
                "aaaa".to_string()
            ]
        ),
        vec![1, 2]
    );
}
