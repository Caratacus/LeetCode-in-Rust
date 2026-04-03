// Tests for Problem 0943: Find the Shortest Superstring
// Java reference: src/test/java/g0901_1000/s0943_find_the_shortest_superstring/SolutionTest.java

use leetcode_in_rust::s0943::find_the_shortest_superstring::Solution;

#[test]
fn test_shortest_superstring() {
    let result = Solution::shortest_superstring(vec!["alex".to_string(), "loves".to_string(), "leetcode".to_string()]);
    assert_eq!(result, "alexlovesleetcode");
}

#[test]
fn test_shortest_superstring2() {
    let result = Solution::shortest_superstring(vec!["catg".to_string(), "ctaagt".to_string(), "gcta".to_string(), "ttca".to_string(), "atgcatc".to_string()]);
    assert_eq!(result, "gctaagttcatgcatc");
}
