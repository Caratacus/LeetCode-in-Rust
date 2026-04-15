// Problem 3361: Shift Distance Between Two Strings
// #Medium #Array #String #Prefix_Sum

pub struct Solution;

impl Solution {
    pub fn shift_distance(
        s: String,
        t: String,
        next_cost: Vec<i32>,
        previous_cost: Vec<i32>,
    ) -> i64 {
        let mut costs = [[i64::MAX; 26]; 26];
        // Build costs for next direction
        for i in 0..26 {
            let mut cost = next_cost[i] as i64;
            let mut j = if i == 25 { 0 } else { i + 1 };
            while j != i {
                costs[i][j] = cost;
                cost += next_cost[j] as i64;
                if j == 25 {
                    j = 0;
                } else {
                    j += 1;
                }
            }
        }
        // Build costs for previous direction (take minimum)
        for i in 0..26 {
            let mut cost = previous_cost[i] as i64;
            let mut j = if i == 0 { 25 } else { i - 1 };
            while j != i {
                costs[i][j] = costs[i][j].min(cost);
                cost += previous_cost[j] as i64;
                if j == 0 {
                    j = 26;
                }
                j -= 1;
            }
        }
        let s_bytes = s.as_bytes();
        let t_bytes = t.as_bytes();
        let n = s.len();
        let mut ans: i64 = 0;
        for i in 0..n {
            ans += costs[(s_bytes[i] - b'a') as usize][(t_bytes[i] - b'a') as usize];
        }
        ans
    }
}
