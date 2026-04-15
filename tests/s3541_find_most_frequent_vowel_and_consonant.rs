// Tests for Problem 3541: Find Most Frequent Vowel and Consonant
// Java reference: src/test/java/g3501_3600/s3541_find_most_frequent_vowel_and_consonant/SolutionTest.java

use leetcode_in_rust::s3541::find_most_frequent_vowel_and_consonant::Solution;

#[test]
fn test_max_freq_sum() { assert_eq!(Solution::max_freq_sum("successes".to_string()), 6); }

#[test]
fn test_max_freq_sum2() { assert_eq!(Solution::max_freq_sum("aeiaeia".to_string()), 3); }
