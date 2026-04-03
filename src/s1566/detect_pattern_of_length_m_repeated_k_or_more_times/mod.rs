// Problem 1566: Detect Pattern of Length M Repeated K or More Times
// #Easy #Array #Enumeration
// #Big_O_Time_O(n*m)_Space_O(1)

pub struct Solution;

impl Solution {
    pub fn contains_pattern(arr: Vec<i32>, m: i32, k: i32) -> bool {
        let m = m as usize;
        let k = k as usize;
        let n = arr.len();

        if n < m * k {
            return false;
        }

        for i in 0..=(n - m) {
            let times = Self::count_pattern_repeats(&arr, i, m);
            if times >= k {
                return true;
            }
        }
        false
    }

    fn count_pattern_repeats(arr: &[i32], start: usize, m: usize) -> usize {
        let n = arr.len();
        let mut times = 1;

        let mut j = start + m;
        while j + m <= n {
            if arr[start..start + m] == arr[j..j + m] {
                times += 1;
                j += m;
            } else {
                break;
            }
        }
        times
    }
}
