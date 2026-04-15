// Problem 3373: Maximize the Number of Target Nodes After Connecting Trees II
// #Hard #Depth_First_Search #Breadth_First_Search #Tree
// #2024_12_03_Time_26_ms_(98.75%)_Space_114.7_MB_(80.00%)

use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> Vec<i32> {
        let n = edges1.len() + 1;
        let g1 = Self::pack_u(n, &edges1);
        let m = edges2.len() + 1;
        let g2 = Self::pack_u(m, &edges2);
        let p2 = Self::parents(&g2);
        let mut eo2 = vec![0i32; 2];
        for i in 0..m {
            eo2[(p2[2][i] % 2) as usize] += 1;
        }
        let max = *eo2.iter().max().unwrap();
        let p1 = Self::parents(&g1);
        let mut eo1 = vec![0i32; 2];
        for i in 0..n {
            eo1[(p1[2][i] % 2) as usize] += 1;
        }
        let mut ans = vec![0; n];
        for i in 0..n {
            ans[i] = eo1[(p1[2][i] % 2) as usize] + max;
        }
        ans
    }

    fn parents(g: &[VecDeque<usize>]) -> Vec<Vec<i32>> {
        let n = g.len();
        let mut par = vec![-1i32; n];
        let mut depth = vec![0i32; n];
        depth[0] = 0;
        let mut q = VecDeque::with_capacity(n);
        q.push_back(0);
        while let Some(cur) = q.pop_front() {
            for &nex in &g[cur] {
                if par[cur] != nex as i32 {
                    q.push_back(nex);
                    par[nex] = cur as i32;
                    depth[nex] = depth[cur] + 1;
                }
            }
        }
        let order: Vec<i32> = q.into_iter().map(|x| x as i32).collect();
        vec![par, order, depth]
    }

    fn pack_u(n: usize, ft: &[Vec<i32>]) -> Vec<VecDeque<usize>> {
        let mut g = vec![VecDeque::with_capacity(0); n];
        let mut p = vec![0usize; n];
        for u in ft {
            p[u[0] as usize] += 1;
            p[u[1] as usize] += 1;
        }
        for i in 0..n {
            g[i].reserve(p[i]);
        }
        for u in ft {
            g[u[0] as usize].push_back(u[1] as usize);
            g[u[1] as usize].push_back(u[0] as usize);
        }
        g
    }
}
