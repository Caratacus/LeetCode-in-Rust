// Problem 3372: Maximize the Number of Target Nodes After Connecting Trees I
// #Medium #Depth_First_Search #Breadth_First_Search #Tree
// #2024_12_03_Time_50_ms_(99.49%)_Space_75.7_MB_(5.10%)

use std::vec::Vec;

pub struct Solution;

impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let a = Self::get(&edges1, k);
        let b = Self::get(&edges2, k);
        let n = a.len();
        let m = b.len();
        let mut ans = vec![0; n];
        let mut max_val = 0;
        for i in 0..m {
            if k != 0 && b[i][k as usize - 1] > max_val {
                max_val = b[i][k as usize - 1];
            }
        }
        for i in 0..n {
            ans[i] = a[i][k as usize] + max_val;
        }
        ans
    }

    fn get_graph(edges: &[Vec<i32>]) -> Vec<Vec<usize>> {
        let n = edges.len() + 1;
        let mut graph: Vec<Vec<usize>> = vec![vec![]; n];
        for edge in edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            graph[u].push(v);
            graph[v].push(u);
        }
        graph
    }

    fn dfs(graph: &[Vec<usize>], u: usize, pt: i32, dp: &mut [Vec<i32>], k: i32) {
        for &v in &graph[u] {
            if v == pt as usize {
                continue;
            }
            Self::dfs(graph, v, u as i32, dp, k);
            for i in 0..k as usize {
                dp[u][i + 1] += dp[v][i];
            }
        }
        dp[u][0] += 1;
    }

    fn dfs2(
        graph: &[Vec<usize>],
        u: usize,
        pt: i32,
        ptv: &[i32],
        fdp: &mut [Vec<i32>],
        dp: &[Vec<i32>],
        k: i32,
    ) {
        fdp[u][0] = dp[u][0];
        for i in 1..=k as usize {
            fdp[u][i] = dp[u][i] + ptv[i - 1];
        }
        for &v in &graph[u] {
            if v == pt as usize {
                continue;
            }
            let mut nptv = vec![0; k as usize + 1];
            for i in 0..k as usize {
                nptv[i + 1] = dp[u][i + 1] - dp[v][i] + ptv[i];
            }
            nptv[0] = 1;
            Self::dfs2(graph, v, u as i32, &nptv, fdp, dp, k);
        }
    }

    fn get(edges: &[Vec<i32>], k: i32) -> Vec<Vec<i32>> {
        let graph = Self::get_graph(edges);
        let n = graph.len();
        let mut dp = vec![vec![0; k as usize + 1]; n];
        let mut fdp = vec![vec![0; k as usize + 1]; n];
        Self::dfs(&graph, 0, -1, &mut dp, k);
        Self::dfs2(&graph, 0, -1, &vec![0; k as usize + 1], &mut fdp, &dp, k);
        for i in 0..n {
            for j in 1..=k as usize {
                fdp[i][j] += fdp[i][j - 1];
            }
        }
        fdp
    }
}
