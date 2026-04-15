// Tests for Problem 3577: Count the Number of Computer Unlocking Permutations
// Java reference: src/test/java/g3501_3600/s3577_count_the_number_of_computer_unlocking_permutations/SolutionTest.java
use leetcode_in_rust::s3577::count_the_number_of_computer_unlocking_permutations::Solution;
#[test] fn test_count_permutations() { assert_eq!(Solution::count_permutations(vec![1, 2, 3]), 2); }
#[test] fn test_count_permutations2() { assert_eq!(Solution::count_permutations(vec![3, 3, 3, 4, 4, 4]), 0); }
