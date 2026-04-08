// Problem 3160: find the number of distinct colors among the balls
// #Medium #Array #Hash_Table #Simulation #2024_05_30_Time_36_ms_(98.82%)_Space_79.6_MB_(93.03%)

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn query_results(_ignored_limit: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ball_to_color: HashMap<i32, i32> = HashMap::new();
        let mut color_to_cnt: HashMap<i32, i32> = HashMap::new();
        let mut ret = Vec::with_capacity(queries.len());

        for query in &queries {
            let ball = query[0];
            let color = query[1];

            if let Some(&old_color) = ball_to_color.get(&ball) {
                let old_color_cnt = color_to_cnt.get(&old_color).copied().unwrap_or(0);
                if old_color_cnt >= 2 {
                    color_to_cnt.insert(old_color, old_color_cnt - 1);
                } else {
                    color_to_cnt.remove(&old_color);
                }
            }

            ball_to_color.insert(ball, color);
            *color_to_cnt.entry(color).or_insert(0) += 1;
            ret.push(color_to_cnt.len() as i32);
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_results() {
        assert_eq!(
            Solution::query_results(4, vec![vec![1, 4], vec![2, 5], vec![1, 3], vec![3, 4]]),
            vec![1, 2, 2, 3]
        );
    }

    #[test]
    fn test_query_results2() {
        assert_eq!(
            Solution::query_results(
                4,
                vec![vec![0, 1], vec![1, 2], vec![2, 2], vec![3, 4], vec![4, 5]]
            ),
            vec![1, 2, 2, 3, 4]
        );
    }

    #[test]
    fn test_query_results3() {
        assert_eq!(
            Solution::query_results(
                1,
                vec![vec![0, 2], vec![1, 10], vec![0, 10], vec![0, 3], vec![1, 5]]
            ),
            vec![1, 2, 1, 2, 2]
        );
    }
}
