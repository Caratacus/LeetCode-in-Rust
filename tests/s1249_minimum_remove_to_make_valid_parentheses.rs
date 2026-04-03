// Tests for Problem 1249: Minimum Remove to Make Valid Parentheses
// Java reference: src/test/java/g1201_1300/s1249_minimum_remove_to_make_valid_parentheses/SolutionTest.java

use leetcode_in_rust::s1249::minimum_remove_to_make_valid_parentheses::Solution;

fn is_valid_parentheses(s: &str) -> bool {
    let mut count = 0;
    for c in s.chars() {
        if c == '(' {
            count += 1;
        } else if c == ')' {
            if count == 0 {
                return false;
            }
            count -= 1;
        }
    }
    count == 0
}

#[test]
fn test_min_remove_to_make_valid() {
    let result = Solution::min_remove_to_make_valid("lee(t(c)o)de)".to_string());
    assert!(is_valid_parentheses(&result));
}

#[test]
fn test_min_remove_to_make_valid2() {
    let result = Solution::min_remove_to_make_valid("a)b(c)d".to_string());
    assert!(is_valid_parentheses(&result));
}

#[test]
fn test_min_remove_to_make_valid3() {
    let result = Solution::min_remove_to_make_valid("))((".to_string());
    assert!(is_valid_parentheses(&result));
}
