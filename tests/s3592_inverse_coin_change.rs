// Tests for Problem 3592: Inverse Coin Change
// Java reference: src/test/java/g3501_3600/s3592_inverse_coin_change/SolutionTest.java
use leetcode_in_rust::s3592::inverse_coin_change::Solution;
#[test] fn test_find_coins() { assert_eq!(Solution::find_coins(vec![0, 1, 0, 2, 0, 3, 0, 4, 0, 5]), vec![2, 4, 6]); }
#[test] fn test_find_coins2() { assert_eq!(Solution::find_coins(vec![1, 2, 2, 3, 4]), vec![1, 2, 5]); }
#[test] fn test_find_coins3() { assert_eq!(Solution::find_coins(vec![1, 2, 3, 4, 15]), vec![]); }
