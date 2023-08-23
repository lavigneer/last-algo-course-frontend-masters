use std::collections::VecDeque;

#[derive(Debug)]
struct BinaryNode<'a, T> {
    value: &'a T,
    left: Option<Box<BinaryNode<'a, T>>>,
    right: Option<Box<BinaryNode<'a, T>>>,
}

fn pre_order_walk<'a, T>(curr: Option<&BinaryNode<'a, T>>, mut path: Vec<&'a T>) -> Vec<&'a T> {
    match curr {
        None => path,
        Some(node) => {
            path.push(node.value);
            path = pre_order_walk(node.left.as_deref(), path);
            path = pre_order_walk(node.right.as_deref(), path);
            path
        }
    }
}

fn post_order_walk<'a, T>(curr: Option<&BinaryNode<'a, T>>, mut path: Vec<&'a T>) -> Vec<&'a T> {
    match curr {
        None => path,
        Some(node) => {
            path = post_order_walk(node.left.as_deref(), path);
            path = post_order_walk(node.right.as_deref(), path);
            path.push(node.value);
            path
        }
    }
}

fn in_order_walk<'a, T>(curr: Option<&BinaryNode<'a, T>>, mut path: Vec<&'a T>) -> Vec<&'a T> {
    match curr {
        None => path,
        Some(node) => {
            path = in_order_walk(node.left.as_deref(), path);
            path.push(node.value);
            path = in_order_walk(node.right.as_deref(), path);
            path
        }
    }
}

fn depth_first_search<'a, T: Eq + PartialEq + Ord + PartialOrd>(
    curr: Option<&BinaryNode<'a, T>>,
    needle: &'a T,
) -> bool {
    match curr {
        None => false,
        Some(node) if node.value == needle => true,
        Some(node) if node.value > needle => depth_first_search(node.left.as_deref(), needle),
        Some(node) if node.value < needle => depth_first_search(node.right.as_deref(), needle),
        _ => unreachable!("Something went wrong here..."),
    }
}

impl<'a, T: Eq + PartialEq + Ord + PartialOrd> BinaryNode<'a, T> {
    pub fn pre_order(&self) -> Vec<&T> {
        pre_order_walk(Some(self), vec![])
    }

    pub fn post_order(&self) -> Vec<&T> {
        post_order_walk(Some(self), vec![])
    }

    pub fn in_order(&self) -> Vec<&T> {
        in_order_walk(Some(self), vec![])
    }

    pub fn breadth_first_search(&'a self, needle: &T) -> bool {
        let mut queue: VecDeque<&'a BinaryNode<'a, T>> = VecDeque::from([self]);

        while let Some(curr) = queue.pop_front() {
            if curr.value == needle {
                return true;
            }
            if let Some(left) = &curr.left {
                queue.push_back(left);
            }
            if let Some(right) = &curr.right {
                queue.push_back(right);
            }
        }

        false
    }

    pub fn depth_first_search(&self, needle: &T) -> bool {
        depth_first_search(Some(self), needle)
    }
}

impl<'a, T: Eq> Eq for BinaryNode<'a, T> {}

impl<'a, T: PartialEq> PartialEq for BinaryNode<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.left == other.left && self.right == other.right
    }
}

impl<'a> Default for BinaryNode<'a, i32> {
    fn default() -> Self {
        Self {
            value: &20,
            right: Some(Box::new(BinaryNode {
                value: &50,
                right: Some(Box::new(BinaryNode {
                    value: &100,
                    right: None,
                    left: None,
                })),
                left: Some(Box::new(BinaryNode {
                    value: &30,
                    right: Some(Box::new(BinaryNode {
                        value: &45,
                        right: None,
                        left: None,
                    })),
                    left: Some(Box::new(BinaryNode {
                        value: &29,
                        right: None,
                        left: None,
                    })),
                })),
            })),
            left: Some(Box::new(BinaryNode {
                value: &10,
                right: Some(Box::new(BinaryNode {
                    value: &15,
                    right: None,
                    left: None,
                })),
                left: Some(Box::new(BinaryNode {
                    value: &5,
                    right: Some(Box::new(BinaryNode {
                        value: &7,
                        right: None,
                        left: None,
                    })),
                    left: None,
                })),
            })),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_order() {
        let tree: BinaryNode<i32> = Default::default();
        let order = tree.in_order();
        assert_eq!(order, vec![&5, &7, &10, &15, &20, &29, &30, &45, &50, &100])
    }

    #[test]
    fn post_order() {
        let tree: BinaryNode<i32> = Default::default();
        let order = tree.post_order();
        assert_eq!(order, vec![&7, &5, &15, &10, &29, &45, &30, &100, &50, &20])
    }

    #[test]
    fn pre_order() {
        let tree: BinaryNode<i32> = Default::default();
        let order = tree.pre_order();
        assert_eq!(order, vec![&20, &10, &5, &7, &15, &50, &30, &29, &45, &100])
    }

    #[test]
    fn bfs() {
        let tree: BinaryNode<i32> = Default::default();
        assert!(tree.breadth_first_search(&45));
        assert!(tree.breadth_first_search(&7));
        assert!(!tree.breadth_first_search(&69));
    }

    #[test]
    fn dfs() {
        let tree: BinaryNode<i32> = Default::default();
        assert!(tree.depth_first_search(&45));
        assert!(tree.depth_first_search(&7));
        assert!(!tree.depth_first_search(&69));
    }

    #[test]
    fn compare() {
        let tree: BinaryNode<i32> = Default::default();
        let tree2 = BinaryNode {
            value: &20,
            right: Some(Box::new(BinaryNode {
                value: &50,
                right: None,
                left: Some(Box::new(BinaryNode {
                    value: &30,
                    right: Some(Box::new(BinaryNode {
                        value: &45,
                        right: Some(Box::new(BinaryNode {
                            value: &49,
                            left: None,
                            right: None,
                        })),
                        left: None,
                    })),
                    left: Some(Box::new(BinaryNode {
                        value: &29,
                        right: None,
                        left: None,
                    })),
                })),
            })),
            left: Some(Box::new(BinaryNode {
                value: &10,
                right: Some(Box::new(BinaryNode {
                    value: &15,
                    right: None,
                    left: None,
                })),
                left: Some(Box::new(BinaryNode {
                    value: &5,
                    right: Some(Box::new(BinaryNode {
                        value: &7,
                        right: None,
                        left: None,
                    })),
                    left: None,
                })),
            })),
        };
        assert_eq!(tree, tree);
        assert_ne!(tree, tree2);
    }
}
