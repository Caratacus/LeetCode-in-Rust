// Tests for Problem 3075: Maximize Happiness of Selected Children
// Java reference: src/test/java/g3001_3100/s3075_maximize_happiness_of_selected_children/SolutionTest.java

use leetcode_in_rust::s3075::maximize_happiness_of_selected_children::Solution;

#[test]
fn test_maximum_happiness_sum() {
    assert_eq!(Solution::maximum_happiness_sum(vec![1, 2, 3], 2), 4);
}

#[test]
fn test_maximum_happiness_sum2() {
    assert_eq!(Solution::maximum_happiness_sum(vec![1, 1, 1, 1], 2), 1);
}

#[test]
fn test_maximum_happiness_sum3() {
    assert_eq!(Solution::maximum_happiness_sum(vec![2, 3, 4, 5], 1), 5);
}
