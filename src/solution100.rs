use std::cell::RefCell;
//https://leetcode.com/problems/same-tree
use std::rc::Rc;

struct Solution;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
    #[inline]
    pub fn new_with_nodes(val: i32, left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> Self {
        TreeNode {
            val,
            left,
            right,
        }
    }
}

impl Solution {
    pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if p.is_none() && q.is_none() {
            return true;
        }
        if p.is_none() || q.is_none() {
            return false;
        }
        let p_unwrapped = p.unwrap();
        let q_unwrapped = q.unwrap();
        let pr = p_unwrapped.borrow();
        let qr = q_unwrapped.borrow();
        return pr.val == qr.val
            && Solution::is_same_tree(pr.left.clone(), qr.left.clone())
            && Solution::is_same_tree(pr.right.clone(), qr.right.clone());
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::solution100::{Solution, TreeNode};

    #[test]
    fn test() {
        let n2 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let n3 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let n1 = Some(Rc::new(RefCell::new(TreeNode::new_with_nodes(1, n2, n3))));
        let n22 = Some(Rc::new(RefCell::new(TreeNode::new(2))));
        let n32 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        let n12 = Some(Rc::new(RefCell::new(TreeNode::new_with_nodes(1, n22, n32))));
        assert_eq!(Solution::is_same_tree(n1, n12), true);
    }
}