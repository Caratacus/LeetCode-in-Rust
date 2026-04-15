// Tests for Problem 3722: Lexicographically Smallest String After Reverse
// Java reference: src/test/java/g3701_3800/s3722_lexicographically_smallest_string_after_reverse/SolutionTest.java
use leetcode_in_rust::s3722::lexicographically_smallest_string_after_reverse::Solution;
#[test]
fn test_lex_smallest() { assert_eq!(Solution::lex_smallest("dcab".to_string()), "acdb".to_string()); }
#[test]
fn test_lex_smallest2() { assert_eq!(Solution::lex_smallest("abba".to_string()), "aabb".to_string()); }
#[test]
fn test_lex_smallest3() { assert_eq!(Solution::lex_smallest("zxy".to_string()), "xzy".to_string()); }
