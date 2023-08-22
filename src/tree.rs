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

impl<'a, T> BinaryNode<'a, T> {
    pub fn pre_order(&self) -> Vec<&T> {
        pre_order_walk(Some(self), vec![])
    }

    pub fn post_order(&self) -> Vec<&T> {
        post_order_walk(Some(self), vec![])
    }

    pub fn in_order(&self) -> Vec<&T> {
        in_order_walk(Some(self), vec![])
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
}
