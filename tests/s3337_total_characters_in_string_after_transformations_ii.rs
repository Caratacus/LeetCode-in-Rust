// Tests for Problem 3337: Total Characters in String After Transformations II
// Java reference: src/test/java/g3301_3400/s3337_total_characters_in_string_after_transformations_ii/SolutionTest.java

use leetcode_in_rust::s3337::total_characters_in_string_after_transformations_ii::Solution;

#[test]
fn test_length_after_transformations() {
    let nums = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2];
    assert_eq!(Solution::length_after_transformations("abcyy".to_string(), 2, nums), 7);
}

#[test]
fn test_length_after_transformations2() {
    let nums = vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2];
    assert_eq!(Solution::length_after_transformations("azbk".to_string(), 1, nums), 8);
}

#[test]
fn test_length_after_transformations3() {
    let nums = vec![9, 1, 6, 3, 2, 7, 8, 10, 8, 3, 9, 5, 10, 8, 10, 2, 2, 9, 10, 1, 3, 5, 4, 4, 8, 10];
    assert_eq!(
        Solution::length_after_transformations(
            "sutnqlhkolxwjtrunkmaakgfyitzluklnrglpbnknbpdvxccpyupjzqldm".to_string(),
            2826,
            nums
        ),
        557232981
    );
}

#[test]
fn test_length_after_transformations4() {
    let nums = vec![5, 3, 8, 1, 4, 2, 2, 4, 5, 2, 8, 5, 8, 2, 6, 10, 8, 1, 4, 1, 7, 4, 2, 4, 7, 5];
    assert_eq!(
        Solution::length_after_transformations(
            "mppgvcssluzhipednraxbdfbyn".to_string(),
            3719,
            nums
        ),
        467065288
    );
}
