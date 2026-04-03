// Tests for Problem 1032: Stream of Characters
// Java reference: src/test/java/g1001_1100/s1032_stream_of_characters/SolutionTest.java

use leetcode_in_rust::s1032::stream_of_characters::StreamChecker;

#[test]
fn test_stream_checker() {
    let mut checker = StreamChecker::new(vec!["cd".to_string(), "f".to_string(), "kl".to_string()]);
    assert_eq!(checker.query('a'), false);
    assert_eq!(checker.query('b'), false);
    assert_eq!(checker.query('c'), false);
    assert_eq!(checker.query('d'), true);
    assert_eq!(checker.query('e'), false);
    assert_eq!(checker.query('f'), true);
    assert_eq!(checker.query('g'), false);
    assert_eq!(checker.query('h'), false);
    assert_eq!(checker.query('i'), false);
    assert_eq!(checker.query('j'), false);
    assert_eq!(checker.query('k'), false);
    assert_eq!(checker.query('l'), true);
}
