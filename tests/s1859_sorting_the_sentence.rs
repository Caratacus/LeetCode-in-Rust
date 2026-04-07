// Tests for Problem 1859: Sorting the Sentence
// Java reference: src/test/java/g1801_1900/s1859_sorting_the_sentence/SolutionTest.java

use leetcode_in_rust::s1859::sorting_the_sentence::Solution;

#[test]
fn test_sort_sentence() {
    assert_eq!(
        Solution::sort_sentence("is2 sentence4 This1 a3".to_string()),
        "This is a sentence".to_string()
    );
}

#[test]
fn test_sort_sentence2() {
    assert_eq!(
        Solution::sort_sentence("Myself2 Me1 I4 and3".to_string()),
        "Me Myself and I".to_string()
    );
}
