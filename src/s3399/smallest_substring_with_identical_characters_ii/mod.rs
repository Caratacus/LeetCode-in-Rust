// Problem 3399: smallest substring with identical characters ii
// #Hard #Bit_Manipulation #Sliding_Window

pub struct Solution;

impl Solution {
    pub fn min_length(s: String, num_ops: i32) -> i32 {
        let b: Vec<u8> = s.bytes().collect();
        let n = b.len();
        let mut flips1 = 0i32;
        let mut flips2 = 0i32;
        for i in 0..n {
            let e1 = if i % 2 == 0 { b'0' } else { b'1' };
            let e2 = if i % 2 == 0 { b'1' } else { b'0' };
            if b[i] != e1 {
                flips1 += 1;
            }
            if b[i] != e2 {
                flips2 += 1;
            }
        }
        if flips1.min(flips2) <= num_ops {
            return 1;
        }
        let mut seg: Vec<i32> = Vec::new();
        let mut count = 1i32;
        let mut max = 1i32;
        for i in 1..n {
            if b[i] != b[i - 1] {
                if count != 1 {
                    seg.push(count);
                    max = max.max(count);
                }
                count = 1;
            } else {
                count += 1;
            }
        }
        if count != 1 {
            seg.push(count);
            max = max.max(count);
        }
        let mut l = 2;
        let mut r = max;
        let mut res = max;
        while l <= r {
            let m = l + (r - l) / 2;
            if Self::check(m, &seg, num_ops) {
                r = m - 1;
                res = m;
            } else {
                l = m + 1;
            }
        }
        res
    }

    fn check(sz: i32, seg: &[i32], mut ops: i32) -> bool {
        for &i in seg {
            ops -= i / (sz + 1);
            if ops < 0 {
                return false;
            }
        }
        true
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
    //   assertThat(new Solution().minLength("000", 0), equalTo(3));
    #[test]
    fn test_min_length4() {
        // TODO: 翻译 Java 测试
    }

    // Java: void minLength5()
    //   assertThat(new Solution().minLength("000001", 1), equalTo(2));
    #[test]
    fn test_min_length5() {
        // TODO: 翻译 Java 测试
    }
}
