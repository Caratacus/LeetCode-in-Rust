// Tests for Problem 3725: Count Ways to Choose Coprime Integers From Rows
// Java reference: src/test/java/g3701_3800/s3725_count_ways_to_choose_coprime_integers_from_rows/SolutionTest.java
use leetcode_in_rust::s3725::count_ways_to_choose_coprime_integers_from_rows::Solution;
#[test]
fn test_count_coprime() { assert_eq!(Solution::count_coprime(vec![vec![1, 2], vec![3, 4]]), 3); }
#[test]
fn test_count_coprime2() { assert_eq!(Solution::count_coprime(vec![vec![2, 2], vec![2, 2]]), 0); }
