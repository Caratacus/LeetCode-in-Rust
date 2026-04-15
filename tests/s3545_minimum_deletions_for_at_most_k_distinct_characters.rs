// Tests for Problem 3545: Minimum Deletions for at Most K Distinct Characters
// Java reference: src/test/java/g3501_3600/s3545_minimum_deletions_for_at_most_k_distinct_characters/SolutionTest.java

use leetcode_in_rust::s3545::minimum_deletions_for_at_most_k_distinct_characters::Solution;

#[test]
fn test_min_deletion() { assert_eq!(Solution::min_deletion("abc".to_string(), 2), 1); }
#[test]
fn test_min_deletion2() { assert_eq!(Solution::min_deletion("aabb".to_string(), 2), 0); }
#[test]
fn test_min_deletion3() { assert_eq!(Solution::min_deletion("yyyzz".to_string(), 1), 2); }
