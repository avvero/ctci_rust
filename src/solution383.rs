struct Solution;

// https://leetcode.com/problems/h-index/description/?envType=study-plan-v2&id=top-interview-150
// 3,0,6,1,5
// 0,1,3,5,6

// 0,1
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut cap: [i32; 128] = [0; 128];
        for x in magazine.chars() {
            cap[x as usize] += 1;
        }
        for x in ransom_note.chars() {
            let h = x;
            cap[h as usize] -= 1;
            if cap[h as usize] < 0 {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use crate::solution383::Solution;

    #[test]
    fn test() {
        assert_eq!(Solution::can_construct(String::from("a"), String::from("b")), false);
        assert_eq!(Solution::can_construct(String::from("aa"), String::from("ab")), false);
        assert_eq!(Solution::can_construct(String::from("aa"), String::from("aab")), true);
    }
}