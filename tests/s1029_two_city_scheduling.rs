// Tests for Problem 1029: Two City Scheduling
// Java reference: src/test/java/g1001_1100/s1029_two_city_scheduling/SolutionTest.java

use leetcode_in_rust::s1029::two_city_scheduling::Solution;

#[test]
fn test_two_city_sched_cost() {
    assert_eq!(
        Solution::two_city_sched_cost(vec![vec![10, 20], vec![30, 200], vec![400, 50], vec![30, 20]]),
        110
    );
}

#[test]
fn test_two_city_sched_cost2() {
    assert_eq!(
        Solution::two_city_sched_cost(vec![
            vec![259, 770],
            vec![448, 54],
            vec![926, 667],
            vec![184, 139],
            vec![840, 118],
            vec![577, 469]
        ]),
        1859
    );
}

#[test]
fn test_two_city_sched_cost3() {
    assert_eq!(
        Solution::two_city_sched_cost(vec![
            vec![515, 563],
            vec![451, 713],
            vec![537, 709],
            vec![343, 819],
            vec![855, 779],
            vec![457, 60],
            vec![650, 359],
            vec![631, 42]
        ]),
        3086
    );
}
