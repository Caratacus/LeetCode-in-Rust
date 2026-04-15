// Tests for Problem 3598: Longest Common Prefix Between Adjacent Strings After Removals
// Java reference: src/test/java/g3501_3600/s3598_longest_common_prefix_between_adjacent_strings_after_removals/SolutionTest.java
use leetcode_in_rust::s3598::longest_common_prefix_between_adjacent_strings_after_removals::Solution;
#[test] fn test_longest_common_prefix() { assert_eq!(Solution::longest_common_prefix(vec!["jump".to_string(), "run".to_string(), "run".to_string(), "jump".to_string(), "run".to_string()]), vec![3, 0, 0, 3, 3]); }
#[test] fn test_longest_common_prefix2() { assert_eq!(Solution::longest_common_prefix(vec!["dog".to_string(), "racer".to_string(), "car".to_string()]), vec![0, 0, 0]); }
