// Problem 3398: smallest substring with identical characters i
// #Hard #Array #Binary_Search #Enumeration

pub struct Solution;

impl Solution {
    pub fn min_length(s: String, ops: i32) -> i32 {
        let arr: Vec<char> = s.chars().collect();
        let n = arr.len();
        let mut p1 = ops;
        let mut p2 = ops;
        let mut q = '0';
        let mut w = '1';
        for i in 0..n {
            if arr[i] != q {
                p1 -= 1;
            }
            if arr[i] != w {
                p2 -= 1;
            }
            q = if q == '0' { '1' } else { '0' };
            w = if w == '0' { '1' } else { '0' };
        }
        if p1 >= 0 || p2 >= 0 {
            return 1;
        }
        let mut low = 2;
        let mut high = n;
        let mut ans = 0;
        while low <= high {
            let mid = (low + high) / 2;
            let mut arr2 = arr.clone();
            let mut p = ops;
            let mut c = 1;
            for i in 1..n {
                if arr2[i] == arr2[i - 1] {
                    c += 1;
                } else {
                    c = 1;
                }
                if c > mid {
                    arr2[i - 1] = if arr2[i - 1] == '0' { '1' } else { '0' };
                    p -= 1;
                    c = 0;
                }
            }
            if p < 0 {
                low = mid + 1;
            } else {
                ans = mid;
                high = mid - 1;
            }
        }
        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Java: void minLength()
    //   assertThat(new Solution().minLength("000001", 1), equalTo(2));
    #[test]
    fn test_min_length() {
        // TODO: 翻译 Java 测试
    }

    // Java: void minLength2()
    //   assertThat(new Solution().minLength("0000", 2), equalTo(1));
    #[test]
    fn test_min_length2() {
        // TODO: 翻译 Java 测试
    }

    // Java: void minLength3()
    //   assertThat(new Solution().minLength("0101", 0), equalTo(1));
    #[test]
    fn test_min_length3() {
        // TODO: 翻译 Java 测试
    }

    // Java: void minLength4()
    //   assertThat(new Solution().minLength("000", 2), equalTo(1));
    #[test]
    fn test_min_length4() {
        // TODO: 翻译 Java 测试
    }
}
