// Tests for Problem 0893: Groups of Special Equivalent Strings
// Java reference: src/test/java/g0801_0900/s0893_groups_of_special_equivalent_strings/SolutionTest.java

use leetcode_in_rust::s0893::groups_of_special_equivalent_strings::Solution;

#[test]
fn test_num_special_equiv_groups() {
    assert_eq!(
        Solution::num_special_equiv_groups(vec![
            "abcd".to_string(),
            "cdab".to_string(),
            "cbad".to_string(),
            "xyzz".to_string(),
            "zzxy".to_string(),
            "zzyx".to_string()
        ]),
        3
    );
}

#[test]
fn test_num_special_equiv_groups2() {
    assert_eq!(
        Solution::num_special_equiv_groups(vec![
            "abc".to_string(),
            "acb".to_string(),
            "bac".to_string(),
            "bca".to_string(),
            "cab".to_string(),
            "cba".to_string()
        ]),
        3
    );
}
