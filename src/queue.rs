type Link<T> = *mut Node<T>;

struct Node<T> {
    value: T,
    next: Link<T>,
}

pub struct Queue<T> {
    head: Link<T>,
    tail: *mut Node<T>,
    pub length: usize,
}

impl<T> Queue<T> {
    fn new() -> Queue<T> {
        Queue {
            head: std::ptr::null_mut(),
            tail: std::ptr::null_mut(),
            length: 0,
        }
    }

    fn enqueue(&mut self, val: T) {
        unsafe {
            let new_tail = Box::into_raw(Box::new(Node {
                value: val,
                next: std::ptr::null_mut(),
            }));

            if !self.tail.is_null() {
                (*self.tail).next = new_tail;
            } else {
                self.head = new_tail;
            }
            self.tail = new_tail;
        }
        self.length += 1;
    }

    fn deque(&mut self) -> Option<T> {
        unsafe {
            if self.head.is_null() {
                None
            } else {
                let head = Box::from_raw(self.head);
                self.head = head.next;

                if self.head.is_null() {
                    self.tail = std::ptr::null_mut();
                }
                self.length -= 1;
                Some(head.value)
            }
        }
    }

    fn peek(&self) -> Option<&T> {
        unsafe { self.head.as_ref().map(|node| &node.value) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut list = Queue::new();

        list.enqueue(5);
        list.enqueue(7);
        list.enqueue(9);

        assert_eq!(list.deque(), Some(5));
        assert_eq!(list.length, 2);

        list.enqueue(11);

        assert_eq!(list.deque(), Some(7));
        assert_eq!(list.deque(), Some(9));
        assert_eq!(list.peek(), Some(&11));
        assert_eq!(list.deque(), Some(11));
        assert_eq!(list.deque(), None);
        assert_eq!(list.length, 0);

        list.enqueue(69);
        assert_eq!(list.peek(), Some(&69));
        assert_eq!(list.length, 1);
    }
}
