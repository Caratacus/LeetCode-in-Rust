// Tests for Problem 1255: Maximum Score Words Formed by Letters
// Java reference: src/test/java/g1201_1300/s1255_maximum_score_words_formed_by_letters/SolutionTest.java

use leetcode_in_rust::s1255::maximum_score_words_formed_by_letters::Solution;

#[test]
fn test_max_score_words() {
    assert_eq!(
        Solution::max_score_words(
            vec!["dog".to_string(), "cat".to_string(), "dad".to_string(), "good".to_string()],
            vec!['a', 'a', 'c', 'd', 'd', 'd', 'g', 'o', 'o'],
            vec![
                1, 0, 9, 5, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        ),
        23
    );
}

#[test]
fn test_max_score_words2() {
    assert_eq!(
        Solution::max_score_words(
            vec!["xxxz".to_string(), "ax".to_string(), "bx".to_string(), "cx".to_string()],
            vec!['z', 'a', 'b', 'c', 'x', 'x', 'x'],
            vec![
                4, 4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 10
            ]
        ),
        27
    );
}

#[test]
fn test_max_score_words3() {
    assert_eq!(
        Solution::max_score_words(
            vec!["leetcode".to_string()],
            vec!['l', 'e', 't', 'c', 'o', 'd'],
            vec![
                0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0
            ]
        ),
        0
    );
}
