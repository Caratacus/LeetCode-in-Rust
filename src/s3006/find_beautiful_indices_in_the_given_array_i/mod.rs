// Problem 3006: find beautiful indices in the given array i
// #Medium #String #Binary_Search #Two_Pointers #Hash_Function #String_Matching #Rolling_Hash
// #2024_02_26_Time_8_ms_(95.86%)_Space_45.8_MB_(80.19%)

pub struct Solution;

impl Solution {
    pub fn beautiful_indices(s: String, a: String, b: String, q: i32) -> Vec<i32> {
        let sc: Vec<char> = s.chars().collect();
        let ac: Vec<char> = a.chars().collect();
        let bc: Vec<char> = b.chars().collect();
        let lpsa = Self::get_lps(&ac);
        let lpsb = Self::get_lps(&bc);
        let n = sc.len();
        let b_len = bc.len();
        let a_len = ac.len();
        let mut comp = vec![0i32; n];
        let mut st: Vec<i32> = Vec::new();
        let mut mo: i32 = -(b_len as i32) + 1;
        if bc[0] == sc[0] {
            comp[0] = 1;
            if b_len == 1 {
                st.push(mo);
            }
        }
        for i in 1..n {
            mo += 1;
            if sc[i] == bc[0] {
                comp[i] = 1;
            }
            let mut k = comp[i - 1];
            if k as usize == b_len {
                k = lpsb[k as usize - 1];
            }
            while k > 0 {
                if bc[k as usize] == sc[i] {
                    comp[i] = k + 1;
                    break;
                }
                k = lpsb[k as usize - 1];
            }
            if comp[i] as usize == b_len {
                st.push(mo);
            }
        }
        let mut sia = 0usize;
        mo = -(a_len as i32) + 1;
        let mut ret: Vec<i32> = Vec::new();
        if st.is_empty() {
            return ret;
        }
        if sc[0] == ac[0] {
            comp[0] = 1;
            if a_len == 1 && st[0] <= q {
                ret.push(0);
            }
        } else {
            comp[0] = 0;
        }
        for i in 1..n {
            mo += 1;
            if sc[i] == ac[0] {
                comp[i] = 1;
            } else {
                comp[i] = 0;
            }
            let mut k = comp[i - 1];
            if k as usize == a_len {
                k = lpsa[k as usize - 1];
            }
            while k > 0 {
                if ac[k as usize] == sc[i] {
                    comp[i] = k + 1;
                    break;
                }
                k = lpsa[k as usize - 1];
            }
            if comp[i] as usize == a_len {
                while sia < st.len() && st[sia] + q < mo {
                    sia += 1;
                }
                if sia == st.len() {
                    break;
                }
                if mo >= st[sia] - q && mo <= st[sia] + q {
                    ret.push(mo);
                }
            }
        }
        ret
    }

    fn get_lps(xc: &[char]) -> Vec<i32> {
        let mut r = vec![0i32; xc.len()];
        for i in 1..xc.len() {
            if xc[i] == xc[0] {
                r[i] = 1;
            }
            let mut k = r[i - 1];
            while k > 0 {
                if xc[k as usize] == xc[i] {
                    r[i] = k + 1;
                    break;
                }
                k = r[k as usize - 1];
            }
        }
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beautiful_indices() {
        assert_eq!(
            Solution::beautiful_indices(
                "isawsquirrelnearmysquirrelhouseohmy".to_string(),
                "my".to_string(),
                "squirrel".to_string(),
                15
            ),
            vec![16, 33]
        );
    }

    #[test]
    fn test_beautiful_indices2() {
        assert_eq!(
            Solution::beautiful_indices("abcd".to_string(), "a".to_string(), "a".to_string(), 4),
            vec![0]
        );
    }
}
