// Problem 1577: Number of Ways Where Square of Number Is Equal to Product of Two Numbers
// #Medium #Array #Hash_Table #Math #Two_Pointers
// #Big_O_Time_O(n^2)_Space_O(1)

pub struct Solution;

impl Solution {
    pub fn num_triplets(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> i32 {
        nums1.sort_unstable();
        nums2.sort_unstable();
        Self::count(&nums1, &nums2) + Self::count(&nums2, &nums1)
    }

    fn count(a: &[i32], b: &[i32]) -> i32 {
        let m = b.len();
        let mut count = 0i32;

        for &value in a {
            let x = (value as i64) * (value as i64);
            let mut j = 0usize;
            let mut k = m - 1;

            while j < k {
                let prod = (b[j] as i64) * (b[k] as i64);
                if prod < x {
                    j += 1;
                } else if prod > x {
                    k -= 1;
                } else if b[j] != b[k] {
                    let mut j_new = j;
                    let mut k_new = k;
                    while j_new < m && b[j] == b[j_new] {
                        j_new += 1;
                    }
                    while k_new > 0 && b[k] == b[k_new] {
                        k_new -= 1;
                    }
                    count += ((j_new - j) * (k - k_new)) as i32;
                    j = j_new;
                    k = k_new;
                } else {
                    let q = k - j + 1;
                    count += ((q) * (q - 1) / 2) as i32;
                    break;
                }
            }
        }
        count
    }
}
