// Tests for Problem 0824: Goat Latin
// Java reference: src/test/java/g0801_0900/s0824_goat_latin/SolutionTest.java

use leetcode_in_rust::s0824::goat_latin::Solution;

#[test]
fn test_to_goat_latin() {
    assert_eq!(
        Solution::to_goat_latin("I speak Goat Latin".to_string()),
        "Imaa peaksmaaa oatGmaaaa atinLmaaaaa"
    );
}

#[test]
fn test_to_goat_latin2() {
    assert_eq!(
        Solution::to_goat_latin("The quick brown fox jumped over the lazy dog".to_string()),
        "heTmaa uickqmaaa rownbmaaaa oxfmaaaaa umpedjmaaaaaa overmaaaaaaa hetmaaaaaaaa azylmaaaaaaaaa ogdmaaaaaaaaaa"
    );
}
