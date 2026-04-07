// Tests for Problem 2586: Count the Number of Vowel Strings in Range
// Java reference: src/test/java/g2501_2600/s2586_count_the_number_of_vowel_strings_in_range/SolutionTest.java

use leetcode_in_rust::s2586::count_the_number_of_vowel_strings_in_range::Solution;

#[test]
fn test_vowel_strings() {
    assert_eq!(
        Solution::vowel_strings(
            vec!["are".to_string(), "amy".to_string(), "u".to_string()],
            0,
            2
        ),
        2
    );
}

#[test]
fn test_vowel_strings2() {
    assert_eq!(
        Solution::vowel_strings(
            vec![
                "hey".to_string(),
                "aeo".to_string(),
                "mu".to_string(),
                "ooo".to_string(),
                "artro".to_string()
            ],
            1,
            4
        ),
        3
    );
}
