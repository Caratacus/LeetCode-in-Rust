// Tests for Problem 3557: Find Maximum Number of Non Intersecting Substrings
// Java reference: src/test/java/g3501_3600/s3557_find_maximum_number_of_non_intersecting_substrings/SolutionTest.java
use leetcode_in_rust::s3557::find_maximum_number_of_non_intersecting_substrings::Solution;
#[test] fn test_max_substrings() { assert_eq!(Solution::max_substrings("abcdeafdef".to_string()), 2); }
#[test] fn test_max_substrings2() { assert_eq!(Solution::max_substrings("bcdaaaab".to_string()), 1); }
