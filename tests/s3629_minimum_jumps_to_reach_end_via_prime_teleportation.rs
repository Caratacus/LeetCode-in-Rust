// Tests for Problem 3629: Minimum Jumps to Reach End via Prime Teleportation
// Java reference: src/test/java/g3601_3700/s3629_minimum_jumps_to_reach_end_via_prime_teleportation/SolutionTest.java
use leetcode_in_rust::s3629::minimum_jumps_to_reach_end_via_prime_teleportation::Solution;
#[test]
fn test_min_jumps() { assert_eq!(Solution::min_jumps(vec![1, 2, 4, 6]), 2); }
#[test]
fn test_min_jumps2() { assert_eq!(Solution::min_jumps(vec![2, 3, 4, 7, 9]), 2); }
#[test]
fn test_min_jumps3() { assert_eq!(Solution::min_jumps(vec![4, 6, 5, 8]), 3); }
#[test]
fn test_min_jumps4() {
    assert_eq!(Solution::min_jumps(vec![893, 786, 607, 137, 69, 381, 790, 233, 15, 42, 7, 764, 890, 269, 84, 262, 870, 514, 514, 650, 269, 485, 760, 181, 489, 107, 585, 428, 862, 563]), 21);
}
#[test]
fn test_min_jumps5() { assert_eq!(Solution::min_jumps(vec![4]), 0); }
