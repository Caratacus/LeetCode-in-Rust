// Tests for Problem 2301: Match Substring After Replacement
// Java reference: src/test/java/g2301_2400/s2301_match_substring_after_replacement/SolutionTest.java

use leetcode_in_rust::s2301::match_substring_after_replacement::Solution;

#[test]
fn test_match_replacement() {
    assert_eq!(
        Solution::match_replacement(
            String::from("fool3e7bar"),
            String::from("leet"),
            vec![vec!['e', '3'], vec!['t', '7'], vec!['t', '8']]
        ),
        true
    );
}

#[test]
fn test_match_replacement2() {
    assert_eq!(
        Solution::match_replacement(
            String::from("fooleetbar"),
            String::from("f00l"),
            vec![vec!['o', '0']]
        ),
        false
    );
}

#[test]
fn test_match_replacement3() {
    assert_eq!(
        Solution::match_replacement(
            String::from("Fool33tbaR"),
            String::from("leetd"),
            vec![
                vec!['e', '3'],
                vec!['t', '7'],
                vec!['t', '8'],
                vec!['d', 'b'],
                vec!['p', 'b']
            ]
        ),
        true
    );
}
