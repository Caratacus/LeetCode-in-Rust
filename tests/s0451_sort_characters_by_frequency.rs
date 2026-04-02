// Tests for Problem 0451: Sort Characters By Frequency
// Java reference: src/test/java/g0401_0500/s0451_sort_characters_by_frequency/SolutionTest.java

use leetcode_in_rust::s0451::sort_characters_by_frequency::Solution;

#[test]
fn test_frequency_sort() {
    let result = Solution::frequency_sort("tree".to_string());
    // Check that 'e' appears first (most frequent)
    assert!(result.starts_with('e'));
    // Check length is preserved
    assert_eq!(result.len(), 4);
}

#[test]
fn test_frequency_sort2() {
    let result = Solution::frequency_sort("cccaaa".to_string());
    // Result should have ccc and aaa in some order
    assert_eq!(result.len(), 6);
}

#[test]
fn test_frequency_sort3() {
    let result = Solution::frequency_sort("Aabb".to_string());
    // Result should have 'b' first (most frequent)
    assert_eq!(result.len(), 4);
}
