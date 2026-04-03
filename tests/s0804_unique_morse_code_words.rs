// Tests for Problem 0804: Unique Morse Code Words
// Java reference: src/test/java/g0701_0800/s0804_unique_morse_code_words/SolutionTest.java

use leetcode_in_rust::s0804::unique_morse_code_words::Solution;

#[test]
fn test_unique_morse_representations() {
    assert_eq!(
        Solution::unique_morse_representations(vec![
            "gin".to_string(),
            "zen".to_string(),
            "gig".to_string(),
            "msg".to_string()
        ]),
        2
    );
}

#[test]
fn test_unique_morse_representations2() {
    assert_eq!(Solution::unique_morse_representations(vec!["a".to_string()]), 1);
}
