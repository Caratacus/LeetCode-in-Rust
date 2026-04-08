// Problem 3123: find edges in shortest paths
// #Hard #Depth_First_Search #Breadth_First_Search #Heap_Priority_Queue #Graph #Shortest_Path
// #2024_04_27_Time_24_ms_(100.00%)_Space_75.2_MB_(88.50%)

use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn find_answer(n: i32, edges: Vec<Vec<i32>>) -> Vec<bool> {
        let n = n as usize;
        let m = edges.len();
        let mut edge = vec![0i32; m * 2];
        let mut weight = vec![0i32; m * 2];
        let mut next = vec![-1i32; m * 2];
        let mut head = vec![-1i32; n];
        let mut index: usize = 0;

        let mut add = |u: usize, v: i32, w: i32| {
            edge[index] = v;
            weight[index] = w;
            next[index] = head[u];
            head[u] = index as i32;
            index += 1;
        };

        for local_edge in &edges {
            let u = local_edge[0] as usize;
            let v = local_edge[1];
            let w = local_edge[2];
            add(u, v, w);
            add(v as usize, local_edge[0], w);
        }

        let mut distances = vec![1_000_000_000_000i64; n];
        let mut heap: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();
        distances[0] = 0;
        heap.push(Reverse((0, 0)));
        while let Some(Reverse((distance, u))) = heap.pop() {
            if distance > distances[u] {
                continue;
            }
            if u == n - 1 {
                break;
            }
            let mut local_index = head[u];
            while local_index != -1 {
                let v = edge[local_index as usize] as usize;
                let w = weight[local_index as usize] as i64;
                let new_distance = distance + w;
                if new_distance < distances[v] {
                    distances[v] = new_distance;
                    heap.push(Reverse((new_distance, v)));
                }
                local_index = next[local_index as usize];
            }
        }

        let mut ans = vec![false; m];
        if distances[n - 1] >= 1_000_000_000_000 {
            return ans;
        }
        Self::dfs(&distances, n - 1, -1, &mut ans, &head, &edge, &weight, &next);
        ans
    }

    fn dfs(
        distances: &[i64],
        u: usize,
        pre: i32,
        ans: &mut [bool],
        head: &[i32],
        edge: &[i32],
        weight: &[i32],
        next: &[i32],
    ) {
        let mut local_index = head[u];
        while local_index != -1 {
            let v = edge[local_index as usize] as usize;
            let w = weight[local_index as usize] as i64;
            let i = (local_index as usize) >> 1;
            if distances[v] + w != distances[u] {
                local_index = next[local_index as usize];
                continue;
            }
            ans[i] = true;
            if v as i32 != pre {
                Self::dfs(distances, v, u as i32, ans, head, edge, weight, next);
            }
            local_index = next[local_index as usize];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_answer() {
        let n = 6;
        let edges = vec![
            vec![0, 1, 4],
            vec![0, 2, 1],
            vec![1, 3, 2],
            vec![1, 4, 3],
            vec![1, 5, 1],
            vec![2, 3, 1],
            vec![3, 5, 3],
        ];
        assert_eq!(
            Solution::find_answer(n, edges),
            vec![true, true, true, false, true, true, true]
        );
    }

    #[test]
    fn test_find_answer2() {
        let n = 4;
        let edges = vec![vec![2, 0, 1], vec![0, 1, 1], vec![1, 3, 1]];
        assert_eq!(Solution::find_answer(n, edges), vec![true, true, true]);
    }
}
