// Tests for Problem 1544: Make The String Great
// Java reference: src/test/java/g1501_1600/s1544_make_the_string_great/SolutionTest.java

use leetcode_in_rust::s1544::make_the_string_great::Solution;

#[test]
fn test_make_good() {
    assert_eq!(Solution::make_good("leEeetcode".to_string()), "leetcode");
}

#[test]
fn test_make_good2() {
    assert_eq!(Solution::make_good("abBAcC".to_string()), "");
}

#[test]
fn test_make_good3() {
    assert_eq!(Solution::make_good("s".to_string()), "s");
}
