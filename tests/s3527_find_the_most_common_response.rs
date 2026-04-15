// Tests for Problem 3527: Find the Most Common Response
// Java reference: src/test/java/g3501_3600/s3527_find_the_most_common_response/SolutionTest.java

use leetcode_in_rust::s3527::find_the_most_common_response::Solution;

#[test]
fn test_find_common_response() {
    assert_eq!(
        Solution::find_common_response(vec![
            vec!["good".to_string(), "ok".to_string(), "good".to_string(), "ok".to_string()],
            vec!["ok".to_string(), "bad".to_string(), "good".to_string(), "ok".to_string(), "ok".to_string()],
            vec!["good".to_string()],
            vec!["bad".to_string()],
        ]),
        "good".to_string()
    );
}

#[test]
fn test_find_common_response2() {
    assert_eq!(
        Solution::find_common_response(vec![
            vec!["good".to_string(), "ok".to_string(), "good".to_string()],
            vec!["ok".to_string(), "bad".to_string()],
            vec!["bad".to_string(), "notsure".to_string()],
            vec!["great".to_string(), "good".to_string()],
        ]),
        "bad".to_string()
    );
}

#[test]
fn test_find_common_response3() {
    assert_eq!(
        Solution::find_common_response(vec![
            vec!["fed".to_string(), "vgdb".to_string(), "w".to_string(), "zs".to_string(), "fed".to_string()],
            vec!["f".to_string(), "cz".to_string(), "pah".to_string(), "gj".to_string(), "rpxr".to_string(), "ugyi".to_string()],
            vec!["t".to_string(), "oja".to_string(), "c".to_string()],
            vec!["ni".to_string(), "fed".to_string(), "mcox".to_string(), "a".to_string(), "f".to_string(), "ni".to_string(), "g".to_string()],
            vec!["ybk".to_string(), "xght".to_string(), "jje".to_string()],
        ]),
        "f".to_string()
    );
}
