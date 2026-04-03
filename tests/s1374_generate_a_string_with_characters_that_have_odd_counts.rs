// Tests for Problem 1374: Generate a String with Characters that Have Odd Counts
// Java reference: src/test/java/g1301_1400/s1374_generate_a_string_with_characters_that_have_odd_counts/SolutionTest.java

use leetcode_in_rust::s1374::generate_a_string_with_characters_that_have_odd_counts::Solution;

#[test]
fn test_generate_the_string() {
    let result = Solution::generate_the_string(4);
    assert_eq!(result.len(), 4);
    // All characters should have odd counts
    let mut counts = std::collections::HashMap::new();
    for c in result.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    for count in counts.values() {
        assert!(count % 2 == 1);
    }
}

#[test]
fn test_generate_the_string2() {
    let result = Solution::generate_the_string(2);
    assert_eq!(result.len(), 2);
    let mut counts = std::collections::HashMap::new();
    for c in result.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    for count in counts.values() {
        assert!(count % 2 == 1);
    }
}

#[test]
fn test_generate_the_string3() {
    let result = Solution::generate_the_string(7);
    assert_eq!(result.len(), 7);
    let mut counts = std::collections::HashMap::new();
    for c in result.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    for count in counts.values() {
        assert!(count % 2 == 1);
    }
}
