use std::{cell::RefCell, rc::Rc};

use crate::TreeHelper;

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
}

impl TreeHelper {
    pub fn create_tree_from_list(vals: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut nodes = vec![];

        for val in vals {
            if let Some(v) = val {
                nodes.push(Some(Rc::new(RefCell::new(TreeNode::new(v)))));
            } else {
                nodes.push(None);
            }
        }

        let mut index = 0;
        while index < nodes.len() {
            if let Some(ref node) = nodes[index] {
                let left_index = 2 * index + 1;
                let right_index = 2 * index + 2;

                if left_index < nodes.len() {
                    if let Some(ref left) = nodes[left_index] {
                        node.borrow_mut().left = Some(left.clone());
                    }
                }

                if right_index < nodes.len() {
                    if let Some(ref right) = nodes[right_index] {
                        node.borrow_mut().right = Some(right.clone());
                    }
                }
            }
            index += 1;
        }

        nodes.first().cloned().flatten()
    }
}
