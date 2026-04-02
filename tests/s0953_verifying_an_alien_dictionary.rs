// Tests for Problem 0953: Verifying an Alien Dictionary
// Java reference: src/test/java/g0901_1000/s0953_verifying_an_alien_dictionary/SolutionTest.java

use leetcode_in_rust::s0953::verifying_an_alien_dictionary::Solution;

#[test]
fn test_is_alien_sorted() {
    assert_eq!(
        Solution::is_alien_sorted(
            vec!["hello".to_string(), "leetcode".to_string()],
            "hlabcdefgijkmnopqrstuvwxyz".to_string()
        ),
        true
    );
}

#[test]
fn test_is_alien_sorted2() {
    assert_eq!(
        Solution::is_alien_sorted(
            vec!["word".to_string(), "world".to_string(), "row".to_string()],
            "worldabcefghijkmnpqstuvxyz".to_string()
        ),
        false
    );
}

#[test]
fn test_is_alien_sorted3() {
    assert_eq!(
        Solution::is_alien_sorted(
            vec!["apple".to_string(), "app".to_string()],
            "abcdefghijklmnopqrstuvwxyz".to_string()
        ),
        false
    );
}
