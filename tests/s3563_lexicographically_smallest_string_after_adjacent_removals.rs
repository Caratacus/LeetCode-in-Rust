// Tests for Problem 3563: Lexicographically Smallest String After Adjacent Removals
// Java reference: src/test/java/g3501_3600/s3563_lexicographically_smallest_string_after_adjacent_removals/SolutionTest.java
use leetcode_in_rust::s3563::lexicographically_smallest_string_after_adjacent_removals::Solution;
#[test] fn test_lexicographically_smallest_string() { assert_eq!(Solution::lexicographically_smallest_string("abc".to_string()), "a".to_string()); }
#[test] fn test_lexicographically_smallest_string2() { assert_eq!(Solution::lexicographically_smallest_string("bcda".to_string()), "".to_string()); }
#[test] fn test_lexicographically_smallest_string3() { assert_eq!(Solution::lexicographically_smallest_string("zdce".to_string()), "zdce".to_string()); }
