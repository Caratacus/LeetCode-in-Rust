// Tests for Problem 3561: Resulting String After Adjacent Removals
// Java reference: src/test/java/g3501_3600/s3561_resulting_string_after_adjacent_removals/SolutionTest.java
use leetcode_in_rust::s3561::resulting_string_after_adjacent_removals::Solution;
#[test] fn test_resulting_string() { assert_eq!(Solution::resulting_string("abc".to_string()), "c".to_string()); }
#[test] fn test_resulting_string2() { assert_eq!(Solution::resulting_string("adcb".to_string()), "".to_string()); }
#[test] fn test_resulting_string3() { assert_eq!(Solution::resulting_string("zadb".to_string()), "db".to_string()); }
