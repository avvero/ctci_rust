struct Solution;

// https://leetcode.com/problems/h-index/description/?envType=study-plan-v2&id=top-interview-150
// 3,0,6,1,5
// 0,1,3,5,6

// 0,1
impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let sub = s.chars().collect::<Vec<char>>();
        let mut i :usize = 0;
        for c in t.chars() {
            if sub.len() == i {
                break;
            }
            if c == sub[i] {
                i = i + 1;
            }
        }
        return i >= s.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::solution392::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::is_subsequence(String::from("abc"), String::from("ahbgdc")), true);
        assert_eq!(Solution::is_subsequence(String::from("axc"), String::from("ahbgdc")), false);
        assert_eq!(Solution::is_subsequence(String::from("b"), String::from("abc")), true);
    }
}