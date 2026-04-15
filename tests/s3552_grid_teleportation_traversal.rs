// Tests for Problem 3552: Grid Teleportation Traversal
// Java reference: src/test/java/g3501_3600/s3552_grid_teleportation_traversal/SolutionTest.java

use leetcode_in_rust::s3552::grid_teleportation_traversal::Solution;

#[test]
fn test_min_moves() { assert_eq!(Solution::min_moves(vec!["A..".to_string(), ".A.".to_string(), "...".to_string()]), 2); }
#[test]
fn test_min_moves2() { assert_eq!(Solution::min_moves(vec![".#...".to_string(), ".#.#.".to_string(), ".#.#.".to_string(), "...#.".to_string()]), 13); }
#[test]
fn test_min_moves3() { assert_eq!(Solution::min_moves(vec![".".to_string(), "A".to_string()]), 1); }
#[test]
fn test_min_moves4() { assert_eq!(Solution::min_moves(vec![".D".to_string(), "EH".to_string()]), 2); }
#[test]
fn test_min_moves5() { assert_eq!(Solution::min_moves(vec![".".to_string()]), 0); }
#[test]
fn test_min_moves6() { assert_eq!(Solution::min_moves(vec![".".to_string(), "#".to_string()]), -1); }
