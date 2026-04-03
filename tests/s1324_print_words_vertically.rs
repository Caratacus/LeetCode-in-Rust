// Tests for Problem 1324: Print Words Vertically
// Java reference: src/test/java/g1301_1400/s1324_print_words_vertically/SolutionTest.java

use leetcode_in_rust::s1324::print_words_vertically::Solution;

#[test]
fn test_print_vertically() {
    assert_eq!(
        Solution::print_vertically("HOW ARE YOU".to_string()),
        vec!["HAY".to_string(), "ORO".to_string(), "WEU".to_string()]
    );
}

#[test]
fn test_print_vertically2() {
    assert_eq!(
        Solution::print_vertically("TO BE OR NOT TO BE".to_string()),
        vec!["TBONTB".to_string(), "OEROOE".to_string(), "   T".to_string()]
    );
}

#[test]
fn test_print_vertically3() {
    assert_eq!(
        Solution::print_vertically("CONTEST IS COMING".to_string()),
        vec![
            "CIC".to_string(),
            "OSO".to_string(),
            "N M".to_string(),
            "T I".to_string(),
            "E N".to_string(),
            "S G".to_string(),
            "T".to_string()
        ]
    );
}
