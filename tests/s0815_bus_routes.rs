// Tests for Problem 0815: Bus Routes
// Java reference: src/test/java/g0701_0800/s0815_bus_routes/SolutionTest.java

use leetcode_in_rust::s0815::bus_routes::Solution;

#[test]
fn test_num_buses_to_destination() {
    assert_eq!(
        Solution::num_buses_to_destination(
            vec![vec![1, 2, 7], vec![3, 6, 7]],
            1,
            6
        ),
        2
    );
}

#[test]
fn test_num_buses_to_destination2() {
    assert_eq!(
        Solution::num_buses_to_destination(
            vec![
                vec![7, 12],
                vec![4, 5, 15],
                vec![6],
                vec![15, 19],
                vec![9, 12, 13],
            ],
            15,
            12
        ),
        -1
    );
}
