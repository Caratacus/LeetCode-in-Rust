// Problem 3378: Count Connected Components in LCM Graph
// #Hard #Array #Hash_Table #Math #Union_Find #Number_Theory

pub struct Solution;

struct Unionfind {
    parent: Vec<usize>,
    rank: Vec<usize>,
    total_components: usize,
}

impl Unionfind {
    fn new(n: usize) -> Self {
        let mut parent = vec![0; n];
        for i in 0..n {
            parent[i] = i;
        }
        Unionfind {
            parent,
            rank: vec![0; n],
            total_components: n,
        }
    }

    fn find(&mut self, u: usize) -> usize {
        if self.parent[u] == u {
            return u;
        }
        self.parent[u] = self.find(self.parent[u]);
        self.parent[u]
    }

    fn union(&mut self, u: usize, v: usize) {
        let parent_u = self.find(u);
        let parent_v = self.find(v);
        if parent_u != parent_v {
            self.total_components -= 1;
            if self.rank[parent_u] == self.rank[parent_v] {
                self.parent[parent_v] = parent_u;
                self.rank[parent_u] += 1;
            } else if self.rank[parent_u] > self.rank[parent_v] {
                self.parent[parent_v] = parent_u;
            } else {
                self.parent[parent_u] = parent_v;
            }
        }
    }
}

impl Solution {
    pub fn count_components(nums: Vec<i32>, threshold: i32) -> i32 {
        let mut good_nums: Vec<usize> = Vec::new();
        let total_nums = nums.len() as i32;
        for &num in &nums {
            if num <= threshold {
                good_nums.push(num as usize);
            }
        }
        if good_nums.is_empty() {
            return total_nums;
        }
        let mut uf = Unionfind::new(good_nums.len());
        let mut present_elements = vec![-1i32; threshold as usize + 1];
        for i in 0..good_nums.len() {
            present_elements[good_nums[i]] = i as i32;
        }
        for &d in &good_nums {
            let mut i = d;
            while i <= threshold as usize {
                if present_elements[i] == -1 {
                    present_elements[i] = present_elements[d] as i32;
                } else if present_elements[i] != present_elements[d] as i32 {
                    uf.union(present_elements[i] as usize, present_elements[d] as usize);
                }
                i += d;
            }
        }
        uf.total_components as i32 + total_nums - good_nums.len() as i32
    }
}
