// Problem 1578: Minimum Time to Make Rope Colorful
// #Medium #Array #String #Dynamic_Programming #Greedy
// #Big_O_Time_O(n)_Space_O(1)

pub struct Solution;

impl Solution {
    pub fn min_cost(s: String, mut cost: Vec<i32>) -> i32 {
        let str: Vec<char> = s.chars().collect();
        let mut min_cost = 0i32;

        for i in 1..str.len() {
            if str[i] == str[i - 1] {
                // accrue the cost of deletion for the lower duplicate
                min_cost += cost[i].min(cost[i - 1]);
                // keep the cost of the higher duplicate for next iteration
                cost[i] = cost[i].max(cost[i - 1]);
            }
        }
        min_cost
    }
}
