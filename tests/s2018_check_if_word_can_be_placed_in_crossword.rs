// Tests for Problem 2018: Check if Word Can Be Placed in Crossword
// Java reference: src/test/java/g2001_2100/s2018_check_if_word_can_be_placed_in_crossword/SolutionTest.java

use leetcode_in_rust::s2018::check_if_word_can_be_placed_in_crossword::Solution;

#[test]
fn test_place_word_in_crossword() {
    assert_eq!(
        Solution::place_word_in_crossword(
            vec![
                vec!['#', ' ', '#'],
                vec![' ', ' ', '#'],
                vec!['#', 'c', ' ']
            ],
            String::from("abc")
        ),
        true
    );
}

#[test]
fn test_place_word_in_crossword2() {
    assert_eq!(
        Solution::place_word_in_crossword(
            vec![
                vec![' ', '#', 'a'],
                vec![' ', '#', 'c'],
                vec![' ', '#', 'a']
            ],
            String::from("ac")
        ),
        false
    );
}

#[test]
fn test_place_word_in_crossword3() {
    assert_eq!(
        Solution::place_word_in_crossword(
            vec![
                vec!['#', ' ', '#'],
                vec![' ', ' ', '#'],
                vec!['#', ' ', 'c']
            ],
            String::from("ca")
        ),
        true
    );
}
