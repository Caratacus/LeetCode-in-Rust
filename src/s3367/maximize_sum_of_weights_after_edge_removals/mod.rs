// Problem 3367: Maximize Sum of Weights After Edge Removals
// #Hard #Dynamic_Programming #Depth_First_Search #Tree

use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub struct Solution;

impl Solution {
    pub fn maximize_sum_of_weights(edges: Vec<Vec<i32>>, k: i32) -> i64 {
        let n = edges.len() + 1;
        let mut adj: Vec<Vec<(usize, i64)>> = vec![vec![]; n];
        for e in &edges {
            let u = e[0] as usize;
            let v = e[1] as usize;
            let w = e[2] as i64;
            adj[u].push((v, w));
            adj[v].push((u, w));
        }

        fn dfs(v: usize, parent: i32, adj: &[Vec<(usize, i64)>], k: i32) -> [i64; 2] {
            let mut sum: i64 = 0;
            let mut pq: BinaryHeap<Reverse<i64>> = BinaryHeap::new();
            for (w, weight) in &adj[v] {
                if *w as i32 == parent {
                    continue;
                }
                let res = dfs(*w, v as i32, adj, k);
                let max_val = std::cmp::max(weight + res[0], res[1]);
                sum += max_val;
                pq.push(Reverse(max_val - res[1]));
            }
            let mut result: [i64; 2] = [0i64, 0];
            while pq.len() > k as usize {
                sum -= pq.pop().unwrap().0;
            }
            result[1] = sum;
            while pq.len() > k as usize - 1 {
                sum -= pq.pop().unwrap().0;
            }
            result[0] = sum;
            result
        }

        dfs(0, -1, &adj, k)[1]
    }
}
