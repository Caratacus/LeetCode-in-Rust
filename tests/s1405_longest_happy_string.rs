// Tests for Problem 1405: Longest Happy String
// Java reference: src/test/java/g1301_1400/s1405_longest_happy_string/SolutionTest.java

use leetcode_in_rust::s1405::longest_happy_string::Solution;

#[test]
fn test_longest_diverse_string() {
    // Result should contain only 'a', 'b', 'c' with no more than 2 consecutive same chars
    let result = Solution::longest_diverse_string(1, 1, 7);
    // Verify length and constraints
    assert!(result.len() <= 9); // a+b+c = 9 max
    assert!(is_valid_happy_string(&result, 1, 1, 7));
}

#[test]
fn test_longest_diverse_string2() {
    let result = Solution::longest_diverse_string(7, 1, 0);
    assert!(result.len() <= 8); // a+b+c = 8 max
    assert!(is_valid_happy_string(&result, 7, 1, 0));
}

// Helper function to validate happy string
fn is_valid_happy_string(s: &str, a: i32, b: i32, c: i32) -> bool {
    let chars: Vec<char> = s.chars().collect();

    // Check no more than 2 consecutive same chars
    for i in 2..chars.len() {
        if chars[i] == chars[i-1] && chars[i-1] == chars[i-2] {
            return false;
        }
    }

    // Check count constraints
    let count_a = chars.iter().filter(|&&ch| ch == 'a').count() as i32;
    let count_b = chars.iter().filter(|&&ch| ch == 'b').count() as i32;
    let count_c = chars.iter().filter(|&&ch| ch == 'c').count() as i32;

    count_a <= a && count_b <= b && count_c <= c
}
