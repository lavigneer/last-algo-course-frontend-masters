type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    value: T,
    next: Link<T>,
}

pub struct Stack<T> {
    head: Link<T>,
    pub length: usize,
}

impl<T> Stack<T> {
    fn new() -> Stack<T> {
        Stack {
            head: None,
            length: 0,
        }
    }

    fn push(&mut self, val: T) {
        let new_node = Box::new(Node {
            value: val,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self.length += 1;
    }

    fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(head) => {
                self.head = head.next;
                self.length -= 1;
                Some(head.value)
            }
            None => None,
        }
    }

    fn peek(&self) -> Option<&T> {
        match self.head.as_ref() {
            Some(head) => Some(&head.value),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut list = Stack::new();

        list.push(5);
        list.push(7);
        list.push(9);

        assert_eq!(list.pop(), Some(9));
        assert_eq!(list.length, 2);

        list.push(11);

        assert_eq!(list.pop(), Some(11));
        assert_eq!(list.pop(), Some(7));
        assert_eq!(list.peek(), Some(&5));
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), None);
        assert_eq!(list.length, 0);

        list.push(69);
        assert_eq!(list.peek(), Some(&69));
        assert_eq!(list.length, 1);
    }
}
