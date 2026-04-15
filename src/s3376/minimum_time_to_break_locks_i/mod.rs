// Problem 3376: Minimum Time to Break Locks I
// #Medium #Array #Dynamic_Programming #Bit_Manipulation #Backtracking #Bitmask

pub struct Solution;

impl Solution {
    pub fn find_minimum_time(strength: Vec<i32>, k: i32) -> i32 {
        let mut strength_local = strength.clone();
        strength_local.sort();
        let mut res = strength_local[0];
        strength_local.remove(0);
        let mut x = 1;
        while !strength_local.is_empty() {
            x += k;
            let mut next_time = (strength_local[0] - 1) / x + 1;
            let mut can_break = next_time * x;
            let mut index_remove = Self::find_index(&strength_local, can_break);
            if strength_local.len() > 1 {
                let next_time1 = (strength_local[1] - 1) / x + 1;
                let can_break1 = next_time1 * x;
                let index_remove1 = Self::find_index(&strength_local, can_break1);
                if next_time1 + (strength_local[0] - 1) / (x + k)
                    < next_time + (strength_local[1] - 1) / (x + k)
                {
                    next_time = next_time1;
                    index_remove = index_remove1;
                }
            }
            res += next_time;
            strength_local.remove(index_remove);
        }
        res
    }

    fn find_index(strength: &[i32], can_break: i32) -> usize {
        let mut l = 0;
        let mut r = strength.len() - 1;
        let mut res = None;
        while l <= r {
            let mid = (l + r) / 2;
            if strength[mid] <= can_break {
                res = Some(mid);
                l = mid + 1;
            } else {
                r = mid - 1;
            }
        }
        res.unwrap_or(0)
    }
}
