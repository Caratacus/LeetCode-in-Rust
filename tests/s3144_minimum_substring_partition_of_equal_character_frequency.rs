// Tests for Problem 3144: Minimum Substring Partition of Equal Character Frequency
// Java reference: src/test/java/g3101_3200/s3144_minimum_substring_partition_of_equal_character_frequency/SolutionTest.java

use leetcode_in_rust::s3144::minimum_substring_partition_of_equal_character_frequency::Solution;
#[test]
fn test_minimum_substrings_in_partition() {
    assert_eq!(Solution::minimum_substrings_in_partition(String::from("fabccddg")), 3);
}
#[test]
fn test_minimum_substrings_in_partition2() {
    assert_eq!(Solution::minimum_substrings_in_partition(String::from("abababaccddb")), 2);
}
}
