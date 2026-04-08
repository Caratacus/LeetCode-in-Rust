// Problem 3067: count pairs of connectable servers in a weighted tree network
// #Medium #Array #Depth_First_Search #Tree #2024_03_31_Time_69_ms_(99.83%)_Space_45.5_MB_(81.49%)

pub struct Solution;

impl Solution {
    pub fn count_pairs_of_connectable_servers(edges: Vec<Vec<i32>>, signal_speed: i32) -> Vec<i32> {
        let n = edges.len() + 1;
        let mut adj: Vec<Vec<i32>> = vec![vec![]; n];
        for edge in &edges {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let w = edge[2];
            adj[u].push(v as i32);
            adj[v].push(u as i32);
            adj[u].push(w);
            adj[v].push(w);
        }

        let mut res = vec![0; n];
        for i in 0..n {
            if adj[i].len() > 2 {
                let mut al: Vec<i32> = Vec::new();
                for j in (0..adj[i].len()).step_by(2) {
                    let mut cnt = 0;
                    Self::dfs(
                        adj[i][j] as usize,
                        i,
                        adj[i][j + 1],
                        &mut cnt,
                        signal_speed,
                        &adj,
                    );
                    al.push(cnt);
                }
                let mut sum = 0;
                for j in al {
                    res[i] += sum * j;
                    sum += j;
                }
            }
        }
        res
    }

    fn dfs(
        node: usize,
        par: usize,
        sum: i32,
        cnt: &mut i32,
        ss: i32,
        adj: &[Vec<i32>],
    ) {
        if sum % ss == 0 {
            *cnt += 1;
        }
        for i in (0..adj[node].len()).step_by(2) {
            let child = adj[node][i] as usize;
            if child != par {
                Self::dfs(child, node, sum + adj[node][i + 1], cnt, ss, adj);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void countPairsOfConnectableServers()
    //   assertThat(
    //   new Solution()
    //   .countPairsOfConnectableServers(
    //   new int[][] {
    //   {0, 1, 1}, {1, 2, 5}, {2, 3, 13}, {3, 4, 9}, {4, 5, 2}
    //   }, 1),
    //   equalTo(new int[] {0, 4, 6, 6, 4, 0}));
    #[test]
    fn test_count_pairs_of_connectable_servers() {
        assert_eq!(
            Solution::count_pairs_of_connectable_servers(
                vec![
                    vec![0, 1, 1],
                    vec![1, 2, 5],
                    vec![2, 3, 13],
                    vec![3, 4, 9],
                    vec![4, 5, 2]
                ],
                1
            ),
            vec![0, 4, 6, 6, 4, 0]
        );
    }

    // Java: void countPairsOfConnectableServers2()
    //   assertThat(
    //   new Solution()
    //   .countPairsOfConnectableServers(
    //   new int[][] {
    //   {0, 6, 3}, {6, 5, 3}, {0, 3, 1}, {3, 2, 7}, {3, 1, 6}, {3, 4, 2}
    //   }, 3),
    //   equalTo(new int[] {2, 0, 0, 0, 0, 0, 2}));
    #[test]
    fn test_count_pairs_of_connectable_servers2() {
        assert_eq!(
            Solution::count_pairs_of_connectable_servers(
                vec![
                    vec![0, 6, 3],
                    vec![6, 5, 3],
                    vec![0, 3, 1],
                    vec![3, 2, 7],
                    vec![3, 1, 6],
                    vec![3, 4, 2]
                ],
                3
            ),
            vec![2, 0, 0, 0, 0, 0, 2]
        );
    }
}
