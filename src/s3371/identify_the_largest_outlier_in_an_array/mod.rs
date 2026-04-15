// Problem 3371: Identify the Largest Outlier in an Array
// #Medium #Array #Hash_Table #Counting #Enumeration

pub struct Solution;

impl Solution {
    pub fn get_largest_outlier(nums: Vec<i32>) -> i32 {
        let mut cnt = vec![0i32; 2001];
        let mut sum: i32 = 0;
        for &i in &nums {
            sum += i;
            cnt[(i + 1000) as usize] += 1;
        }
        for i in (0..=2000).rev() {
            let j = i as i32 - 1000;
            if cnt[i] == 0 {
                continue;
            }
            sum -= j;
            let csum = (sum / 2) + 1000;
            cnt[i] -= 1;
            if sum % 2 == 0 && csum >= 0 && (csum as usize) < cnt.len() && cnt[csum as usize] > 0 {
                return j;
            }
            sum += j;
            cnt[i] += 1;
        }
        0
    }
}
