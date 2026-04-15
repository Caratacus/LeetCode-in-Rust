// Tests for Problem 3600: Maximize Spanning Tree Stability with Upgrades
// Java reference: src/test/java/g3501_3600/s3600_maximize_spanning_tree_stability_with_upgrades/SolutionTest.java
use leetcode_in_rust::s3600::maximize_spanning_tree_stability_with_upgrades::Solution;
#[test] fn test_max_stability() { assert_eq!(Solution::max_stability(3, vec![vec![0, 1, 2, 1], vec![1, 2, 3, 0]], 1), 2); }
#[test] fn test_max_stability2() { assert_eq!(Solution::max_stability(3, vec![vec![0, 1, 4, 0], vec![1, 2, 3, 0], vec![0, 2, 1, 0]], 2), 6); }
#[test] fn test_max_stability3() { assert_eq!(Solution::max_stability(3, vec![vec![0, 1, 1, 1], vec![1, 2, 1, 1], vec![2, 0, 1, 1]], 0), -1); }
