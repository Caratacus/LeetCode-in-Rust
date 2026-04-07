// Tests for Problem 2211: Count Collisions on a Road
// Java reference: src/test/java/g2201_2300/s2211_count_collisions_on_a_road/SolutionTest.java

use leetcode_in_rust::s2211::count_collisions_on_a_road::Solution;

#[test]
fn test_count_collisions() {
    assert_eq!(Solution::count_collisions("RLRSLL".to_string()), 5);
}

#[test]
fn test_count_collisions2() {
    assert_eq!(Solution::count_collisions("LLRR".to_string()), 0);
}

#[test]
fn test_count_collisions3() {
    assert_eq!(
        Solution::count_collisions("SRRLRLRSRLRSSRRLSLRLLRSLSLLSSRRLSRSLSLRRS".to_string()),
        28
    );
}

#[test]
fn test_count_collisions4() {
    assert_eq!(
        Solution::count_collisions("SSRSSRLLRSLLRSRSSRLRRRRLLRRLSSRR".to_string()),
        20
    );
}
