// Tests for Problem 3583: Count Special Triplets
// Java reference: src/test/java/g3501_3600/s3583_count_special_triplets/SolutionTest.java
use leetcode_in_rust::s3583::count_special_triplets::Solution;
#[test] fn test_special_triplets() { assert_eq!(Solution::special_triplets(vec![6, 3, 6]), 1); }
#[test] fn test_special_triplets2() { assert_eq!(Solution::special_triplets(vec![0, 1, 0, 0]), 1); }
#[test] fn test_special_triplets3() { assert_eq!(Solution::special_triplets(vec![8, 4, 2, 8, 4]), 2); }
