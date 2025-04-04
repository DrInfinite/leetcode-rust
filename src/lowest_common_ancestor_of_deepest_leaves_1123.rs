use std::{cell::RefCell, rc::Rc};

use crate::{Solution, binary_tree_node::TreeNode};

impl Solution {
    pub fn lca_deepest_leaves(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, Option<Rc<RefCell<TreeNode>>>) {
            match node {
                None => (0, None),
                Some(n) => {
                    let node_reference = n.borrow();
                    let (left_height, left_lca) = dfs(node_reference.left.clone());
                    let (right_height, right_lca) = dfs(node_reference.right.clone());

                    match left_height.cmp(&right_height) {
                        std::cmp::Ordering::Equal => (left_height + 1, Some(n.clone())),
                        std::cmp::Ordering::Greater => (left_height + 1, left_lca),
                        std::cmp::Ordering::Less => (right_height + 1, right_lca),
                    }
                }
            }
        }

        return dfs(root).1;
    }
}
