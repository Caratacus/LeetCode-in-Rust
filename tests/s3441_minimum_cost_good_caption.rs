// Tests for Problem 3441: Minimum Cost Good Caption
// Java reference: src/test/java/g3401_3500/s3441_minimum_cost_good_caption/SolutionTest.java

use leetcode_in_rust::s3441::minimum_cost_good_caption::Solution;

#[test]
fn test_min_cost_good_caption() {
    assert_eq!(Solution::min_cost_good_caption("cdcd".to_string()), "cccc".to_string());
}

#[test]
fn test_min_cost_good_caption2() {
    assert_eq!(Solution::min_cost_good_caption("aca".to_string()), "aaa".to_string());
}

#[test]
fn test_min_cost_good_caption3() {
    assert_eq!(Solution::min_cost_good_caption("bc".to_string()), "".to_string());
}

#[test]
fn test_min_cost_good_caption4() {
    assert_eq!(Solution::min_cost_good_caption("antwfdps".to_string()), "nnnnnppp".to_string());
}

#[test]
fn test_min_cost_good_caption5() {
    assert_eq!(Solution::min_cost_good_caption("qzlhsvlf".to_string()), "qqqlllll".to_string());
}

#[test]
fn test_min_cost_good_caption6() {
    assert_eq!(Solution::min_cost_good_caption("qeopwomhpq".to_string()), "oooooooppp".to_string());
}
