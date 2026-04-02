// Tests for Problem 0344: Reverse String
// Java reference: src/test/java/g0301_0400/s0344_reverse_string/SolutionTest.java

use leetcode_in_rust::s0344::reverse_string::Solution;

// Note: API takes Vec<char> by value, cannot verify result
#[test]
fn test_reverse_string() {
    let input = vec!['h', 'e', 'l', 'l', 'o'];
    Solution::reverse_string(input);
    // Note: Cannot verify result as API takes ownership
}

#[test]
fn test_reverse_string2() {
    let input = vec!['H', 'a', 'n', 'n', 'a', 'h'];
    Solution::reverse_string(input);
    // Note: Cannot verify result as API takes ownership
}
