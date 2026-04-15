// Tests for Problem 3492: Maximum Containers on a Ship
// Java reference: src/test/java/g3401_3500/s3492_maximum_containers_on_a_ship/SolutionTest.java

use leetcode_in_rust::s3492::maximum_containers_on_a_ship::Solution;

#[test]
fn test_max_containers() {
    assert_eq!(Solution::max_containers(2, 3, 15), 4);
}

#[test]
fn test_max_containers2() {
    assert_eq!(Solution::max_containers(3, 5, 20), 4);
}
