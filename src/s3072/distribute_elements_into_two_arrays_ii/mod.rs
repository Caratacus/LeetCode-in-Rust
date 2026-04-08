// Problem 3072: distribute elements into two arrays ii
// #Hard #Array #Simulation #Segment_Tree #Binary_Indexed_Tree
// #2024_04_15_Time_48_ms_(99.90%)_Space_65_MB_(74.73%)

pub struct Solution;

struct BIT {
    tree: Vec<i32>,
}

impl BIT {
    fn new(size: usize) -> Self {
        BIT {
            tree: vec![0; size + 1],
        }
    }

    fn update(&mut self, ind: usize) {
        let mut ind = ind;
        while ind < self.tree.len() {
            self.tree[ind] += 1;
            ind += Self::lsb(ind);
        }
    }

    fn rsq(&self, ind: usize) -> i32 {
        let mut sum = 0;
        let mut ind = ind;
        while ind > 0 {
            sum += self.tree[ind];
            ind -= Self::lsb(ind);
        }
        sum
    }

    fn lsb(n: usize) -> usize {
        n & n.wrapping_neg()
    }
}

impl Solution {
    pub fn result_array(source: Vec<i32>) -> Vec<i32> {
        let nums = Self::shrink(&source);
        let mut arr1 = vec![0i32; nums.len()];
        let mut arr2 = vec![0i32; nums.len()];
        arr1[0] = source[0];
        arr2[0] = source[1];
        let mut p1: usize = 0;
        let mut p2: usize = 0;

        let mut bit1 = BIT::new(nums.len());
        bit1.update(nums[0] as usize);
        let mut bit2 = BIT::new(nums.len());
        bit2.update(nums[1] as usize);

        for i in 2..nums.len() {
            let g1 = (p1 + 1) as i32 - bit1.rsq(nums[i] as usize);
            let g2 = (p2 + 1) as i32 - bit2.rsq(nums[i] as usize);
            if g1 < g2 || p1 > p2 {
                p2 += 1;
                arr2[p2] = source[i];
                bit2.update(nums[i] as usize);
            } else {
                p1 += 1;
                arr1[p1] = source[i];
                bit1.update(nums[i] as usize);
            }
        }

        for i in (p1 + 1)..arr1.len() {
            arr1[i] = arr2[i - p1 - 1];
        }
        arr1
    }

    fn shrink(nums: &[i32]) -> Vec<i32> {
        let mut b: Vec<(i64, usize)> = nums
            .iter()
            .enumerate()
            .map(|(i, &v)| ((v as i64) << 32, i))
            .collect();
        b.sort();
        let mut result = vec![0; nums.len()];
        let mut p = 1;
        for i in 0..nums.len() {
            if i > 0 && (b[i].0 ^ b[i - 1].0) >> 32 != 0 {
                p += 1;
            }
            result[b[i].1] = p;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void resultArray()
    //   assertThat(
    //   new Solution().resultArray(new int[] {2, 1, 3, 3}),
    //   equalTo(new int[] {2, 3, 1, 3}));
    #[test]
    fn test_result_array() {
        assert_eq!(Solution::result_array(vec![2, 1, 3, 3]), vec![2, 3, 1, 3]);
    }

    // Java: void resultArray2()
    //   assertThat(
    //   new Solution().resultArray(new int[] {5, 14, 3, 1, 2}),
    //   equalTo(new int[] {5, 3, 2, 14, 1}));
    #[test]
    fn test_result_array2() {
        assert_eq!(
            Solution::result_array(vec![5, 14, 3, 1, 2]),
            vec![5, 3, 2, 14, 1]
        );
    }

    // Java: void resultArray3()
    //   assertThat(
    //   new Solution().resultArray(new int[] {3, 3, 3, 3}),
    //   equalTo(new int[] {3, 3, 3, 3}));
    #[test]
    fn test_result_array3() {
        assert_eq!(Solution::result_array(vec![3, 3, 3, 3]), vec![3, 3, 3, 3]);
    }
}
