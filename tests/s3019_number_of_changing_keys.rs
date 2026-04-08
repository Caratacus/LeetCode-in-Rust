// Tests for Problem 3019: Number of Changing Keys
// Java reference: src/test/java/g3001_3100/s3019_number_of_changing_keys/SolutionTest.java

use leetcode_in_rust::s3019::number_of_changing_keys::Solution;

#[test]
fn test_count_key_changes() {
    assert_eq!(Solution::count_key_changes(String::from("aAbBcC")), 2);
}

#[test]
fn test_count_key_changes2() {
    assert_eq!(Solution::count_key_changes(String::from("AaAaAaaA")), 0);
}
