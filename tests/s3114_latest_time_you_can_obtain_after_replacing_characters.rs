// Tests for Problem 3114: Latest Time You Can Obtain After Replacing Characters
// Java reference: src/test/java/g3101_3200/s3114_latest_time_you_can_obtain_after_replacing_characters/SolutionTest.java

use leetcode_in_rust::s3114::latest_time_you_can_obtain_after_replacing_characters::Solution;

#[test]
fn test_find_latest_time() {
    assert_eq!(Solution::find_latest_time("1?:?4".to_string()), "11:54");
}

#[test]
fn test_find_latest_time2() {
    assert_eq!(Solution::find_latest_time("0?:5?".to_string()), "09:59");
}

#[test]
fn test_find_latest_time3() {
    assert_eq!(Solution::find_latest_time("?1:?6".to_string()), "11:56");
}

#[test]
fn test_find_latest_time4() {
    assert_eq!(Solution::find_latest_time("08:33".to_string()), "08:33");
}

#[test]
fn test_find_latest_time5() {
    assert_eq!(Solution::find_latest_time("??:1?".to_string()), "11:19");
}

#[test]
fn test_find_latest_time6() {
    assert_eq!(Solution::find_latest_time("04:??".to_string()), "04:59");
}

#[test]
fn test_find_latest_time7() {
    assert_eq!(Solution::find_latest_time("?3:12".to_string()), "03:12");
}
