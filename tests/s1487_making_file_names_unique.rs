// Tests for Problem 1487: Making File Names Unique
// Java reference: src/test/java/g1401_1500/s1487_making_file_names_unique/SolutionTest.java

use leetcode_in_rust::s1487::making_file_names_unique::Solution;

#[test]
fn test_get_folder_names() {
    let result = Solution::get_folder_names(vec!["pes".to_string(), "fifa".to_string(), "gta".to_string(), "pes(2019)".to_string()]);
    assert_eq!(result, vec!["pes".to_string(), "fifa".to_string(), "gta".to_string(), "pes(2019)".to_string()]);
}

#[test]
fn test_get_folder_names2() {
    let result = Solution::get_folder_names(vec!["gta".to_string(), "gta(1)".to_string(), "gta".to_string(), "avalon".to_string()]);
    assert_eq!(result, vec!["gta".to_string(), "gta(1)".to_string(), "gta(2)".to_string(), "avalon".to_string()]);
}

#[test]
fn test_get_folder_names3() {
    let result = Solution::get_folder_names(vec!["onepiece".to_string(), "onepiece(1)".to_string(), "onepiece(2)".to_string(), "onepiece(3)".to_string(), "onepiece".to_string()]);
    assert_eq!(result, vec!["onepiece".to_string(), "onepiece(1)".to_string(), "onepiece(2)".to_string(), "onepiece(3)".to_string(), "onepiece(4)".to_string()]);
}
