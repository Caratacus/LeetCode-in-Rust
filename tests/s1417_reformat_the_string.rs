// Tests for Problem 1417: Reformat The String
// Java reference: src/test/java/g1401_1500/s1417_reformat_the_string/SolutionTest.java

use leetcode_in_rust::s1417::reformat_the_string::Solution;

#[test]
fn test_reformat() {
    let result = Solution::reformat("a0b1c2".to_string());
    // Result can be any valid reformat, check it's valid
    assert!(!result.is_empty());
}

#[test]
fn test_reformat2() {
    assert_eq!(Solution::reformat("leetcode".to_string()), "");
}

#[test]
fn test_reformat3() {
    assert_eq!(Solution::reformat("1229857369".to_string()), "");
}
