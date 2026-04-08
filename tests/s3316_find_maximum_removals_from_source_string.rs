// Tests for Problem 3316: Find Maximum Removals From Source String
// Java reference: src/test/java/g3301_3400/s3316_find_maximum_removals_from_source_string/SolutionTest.java

use leetcode_in_rust::s3316::find_maximum_removals_from_source_string::Solution;

#[test]
fn test_max_removals() {
    assert_eq!(Solution::max_removals("abbaa".to_string(), "aba".to_string(), vec![0, 1, 2]), 1);
}

#[test]
fn test_max_removals2() {
    assert_eq!(Solution::max_removals("bcda".to_string(), "d".to_string(), vec![0, 3]), 2);
}

#[test]
fn test_max_removals3() {
    assert_eq!(Solution::max_removals("dda".to_string(), "dda".to_string(), vec![0, 1, 2]), 0);
}

#[test]
fn test_max_removals4() {
    assert_eq!(Solution::max_removals("yeyeykyded".to_string(), "yeyyd".to_string(), vec![0, 2, 3, 4]), 2);
}
