// Problem 3161: block placement queries
// #Hard #Array #Binary_Search #Segment_Tree #Binary_Indexed_Tree
// #2025_03_16_Time_47_ms_(100.00%)_Space_144.38_MB_(56.41%)

pub struct Solution;

impl Solution {
    pub fn get_results(queries: Vec<Vec<i32>>) -> Vec<bool> {
        let m = queries.len();
        let mut pos = vec![0i32; m + 1];
        let mut size = 1;
        let mut max = 0;
        for q in &queries {
            max = max.max(q[1]);
            if q[0] == 1 {
                pos[size] = q[1];
                size += 1;
            }
        }
        pos[..size].sort();
        let max = (max + 1) as usize;
        let mut left = UnionFind::new(max + 1);
        let mut right = UnionFind::new(max + 1);
        let mut bit = BIT::new(max);

        Self::initialize_positions(size, &pos, &mut bit, &mut left, &mut right, max);

        Self::get_booleans(&queries, m, size, &mut left, &mut right, &mut bit)
    }

    fn initialize_positions(
        size: usize,
        pos: &[i32],
        bit: &mut BIT,
        left: &mut UnionFind,
        right: &mut UnionFind,
        max: usize,
    ) {
        for i in 1..size {
            let pre = pos[i - 1] as usize;
            let cur = pos[i] as usize;
            bit.update(cur, cur - pre);
            for j in (pre + 1)..cur {
                left.parent[j] = pre;
                right.parent[j] = cur;
            }
        }
        let last_pos = pos[size - 1] as usize;
        for j in (last_pos + 1)..max {
            left.parent[j] = last_pos;
            right.parent[j] = max;
        }
    }

    fn get_booleans(
        queries: &[Vec<i32>],
        m: usize,
        size: usize,
        left: &mut UnionFind,
        right: &mut UnionFind,
        bit: &mut BIT,
    ) -> Vec<bool> {
        let mut ans = vec![false; m - size + 2];
        let mut index = ans.len() - 1;
        for i in (0..m).rev() {
            let q = &queries[i];
            let x = q[1] as usize;
            let pre = left.find(x - 1);
            if q[0] == 1 {
                let next = right.find(x + 1);
                left.parent[x] = pre;
                right.parent[x] = next;
                bit.update(next, next - pre);
            } else {
                let max_gap = bit.query(pre).max(x - pre);
                index -= 1;
                ans[index] = max_gap >= q[2] as usize;
            }
        }
        ans[index..].to_vec()
    }
}

struct BIT {
    n: usize,
    tree: Vec<usize>,
}

impl BIT {
    fn new(n: usize) -> Self {
        BIT {
            n,
            tree: vec![0; n],
        }
    }

    fn update(&mut self, i: usize, v: usize) {
        let mut i = i;
        while i < self.n {
            self.tree[i] = self.tree[i].max(v);
            i += i & (!i + 1);
        }
    }

    fn query(&self, mut i: usize) -> usize {
        let mut result = 0;
        while i > 0 {
            result = result.max(self.tree[i]);
            i &= i - 1;
        }
        result
    }
}

struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut parent = vec![0; n];
        for i in 1..n {
            parent[i] = i;
        }
        UnionFind { parent }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void getResults()
    //   assertThat(
    //   new Solution().getResults(new int[][] {{1, 2}, {2, 3, 3}, {2, 3, 1}, {2, 2, 2}}),
    //   equalTo(List.of(false, true, true)));
    #[test]
    fn test_get_results() {
        assert_eq!(
            Solution::get_results(vec![vec![1, 2], vec![2, 3, 3], vec![2, 3, 1], vec![2, 2, 2]]),
            vec![false, true, true]
        );
    }

    // Java: void getResults2()
    //   assertThat(
    //   new Solution()
    //   .getResults(new int[][] {{1, 7}, {2, 7, 6}, {1, 2}, {2, 7, 5}, {2, 7, 6}}),
    //   equalTo(List.of(true, true, false)));
    #[test]
    fn test_get_results2() {
        assert_eq!(
            Solution::get_results(vec![
                vec![1, 7],
                vec![2, 7, 6],
                vec![1, 2],
                vec![2, 7, 5],
                vec![2, 7, 6]
            ]),
            vec![true, true, false]
        );
    }

    // Java: void getResults3()
    //   assertThat(
    //   new Solution()
    //   .getResults(
    //   new int[][] {{1, 4}, {1, 9}, {2, 15, 4}, {2, 11, 6}, {2, 13, 10}}),
    //   equalTo(List.of(true, false, false)));
    #[test]
    fn test_get_results3() {
        assert_eq!(
            Solution::get_results(vec![
                vec![1, 4],
                vec![1, 9],
                vec![2, 15, 4],
                vec![2, 11, 6],
                vec![2, 13, 10]
            ]),
            vec![true, false, false]
        );
    }
}
