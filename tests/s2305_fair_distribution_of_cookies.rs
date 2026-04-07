// Tests for Problem 2305: Fair Distribution of Cookies
// Java reference: src/test/java/g2301_2400/s2305_fair_distribution_of_cookies/SolutionTest.java

use leetcode_in_rust::s2305::fair_distribution_of_cookies::Solution;

#[test]
fn test_distribute_cookies() {
    assert_eq!(Solution::distribute_cookies(vec![8, 15, 10, 20, 8], 2), 31);
}

#[test]
fn test_distribute_cookies2() {
    assert_eq!(Solution::distribute_cookies(vec![6, 1, 3, 2, 2, 4, 1, 2], 3), 7);
}
