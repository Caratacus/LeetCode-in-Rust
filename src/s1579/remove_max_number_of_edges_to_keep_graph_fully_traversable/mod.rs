// Problem 1579: Remove Max Number of Edges to Keep Graph Fully Traversable
// #Hard #Graph #Union_Find
// #Big_O_Time_O(m*log(m))_Space_O(n)

pub struct Solution;

impl Solution {
    pub fn max_num_edges_to_remove(n: i32, mut edges: Vec<Vec<i32>>) -> i32 {
        // Sort edges by type in descending order (type 3 first)
        edges.sort_by(|a, b| b[0].cmp(&a[0]));

        let n = n as usize;
        let mut alice: Vec<usize> = (0..=n).collect();
        let mut rank_alice = vec![0usize; n + 1];
        let mut bob: Vec<usize> = (0..=n).collect();
        let mut rank_bob = vec![0usize; n + 1];

        let mut count_alice = n;
        let mut count_bob = n;
        let mut remove = 0;

        for edge in &edges {
            let typ = edge[0] as usize;
            let u = edge[1] as usize;
            let v = edge[2] as usize;

            if typ == 1 {
                if Self::union(u, v, &mut alice, &mut rank_alice) {
                    count_alice -= 1;
                } else {
                    remove += 1;
                }
            } else if typ == 2 {
                if Self::union(u, v, &mut bob, &mut rank_bob) {
                    count_bob -= 1;
                } else {
                    remove += 1;
                }
            } else {
                let b = Self::union(u, v, &mut bob, &mut rank_bob);
                let a = Self::union(u, v, &mut alice, &mut rank_alice);
                if !a && !b {
                    remove += 1;
                }
                if a {
                    count_alice -= 1;
                }
                if b {
                    count_bob -= 1;
                }
            }
        }

        if count_alice != 1 || count_bob != 1 {
            -1
        } else {
            remove as i32
        }
    }

    fn union(x: usize, y: usize, arr: &mut [usize], rank: &mut [usize]) -> bool {
        let p1 = Self::find(x, arr);
        let p2 = Self::find(y, arr);
        if p1 != p2 {
            if rank[p1] > rank[p2] {
                arr[p2] = p1;
            } else if rank[p1] < rank[p2] {
                arr[p1] = p2;
            } else {
                arr[p1] = p2;
                rank[p2] += 1;
            }
            return true;
        }
        false
    }

    fn find(x: usize, arr: &mut [usize]) -> usize {
        if arr[x] == x {
            return x;
        }
        arr[x] = Self::find(arr[x], arr);
        arr[x]
    }
}
