use std::{marker::PhantomData, ptr::NonNull};

type Link<T> = Option<NonNull<Node<T>>>;

struct Node<T> {
    elem: T,
    front: Link<T>,
    back: Link<T>,
}

pub struct DoublyLinkedList<T> {
    front: Link<T>,
    back: Link<T>,
    pub len: usize,
    _boo: PhantomData<T>,
}

pub struct DoublyLinkedListIter<'a, T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    _boo: PhantomData<&'a T>,
}

pub struct DoublyLinkedListIterMut<'a, T> {
    front: Link<T>,
    back: Link<T>,
    len: usize,
    _boo: PhantomData<&'a mut T>,
}

pub struct IntoDoublyLinkedListIter<T> {
    list: DoublyLinkedList<T>,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> DoublyLinkedList<T> {
        DoublyLinkedList {
            front: None,
            back: None,
            len: 0,
            _boo: PhantomData,
        }
    }

    pub fn push_front(&mut self, elem: T) {
        unsafe {
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                front: None,
                back: None,
                elem,
            })));
            if let Some(old) = self.front {
                (*old.as_ptr()).front = Some(new);
                (*new.as_ptr()).back = Some(old);
            } else {
                self.back = Some(new);
            }
            self.front = Some(new);
            self.len += 1;
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        unsafe {
            self.front.map(|node| {
                let boxed_node = Box::from_raw(node.as_ptr());
                let result = boxed_node.elem;

                self.front = boxed_node.back;
                if let Some(new) = self.front {
                    (*new.as_ptr()).front = None;
                } else {
                    self.back = None;
                }

                self.len -= 1;
                result
            })
        }
    }

    pub fn push_back(&mut self, elem: T) {
        unsafe {
            let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                front: None,
                back: None,
                elem,
            })));
            if let Some(old) = self.back {
                (*old.as_ptr()).back = Some(new);
                (*new.as_ptr()).front = Some(old);
            } else {
                self.front = Some(new);
            }
            self.back = Some(new);
            self.len += 1;
        }
    }

    pub fn pop_back(&mut self) -> Option<T> {
        unsafe {
            self.back.map(|node| {
                let boxed_node = Box::from_raw(node.as_ptr());
                let result = boxed_node.elem;

                self.back = boxed_node.front;
                if let Some(new) = self.back {
                    (*new.as_ptr()).back = None;
                } else {
                    self.front = None;
                }

                self.len -= 1;
                result
            })
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    fn get_at(&self, index: usize) -> Link<T> {
        let mut remaining_index = index;
        let mut curr_node = self.front;
        while remaining_index > 0 {
            match curr_node {
                Some(node) => unsafe {
                    curr_node = (*node.as_ptr()).back;
                    remaining_index -= 1;
                },
                None => {
                    return None;
                }
            }
        }
        curr_node
    }

    pub fn remove_at(&mut self, index: usize) {
        if let Some(node_at) = self.get_at(index) {
            unsafe {
                let before_node = (*node_at.as_ptr()).back;
                let after_node = (*node_at.as_ptr()).front;
                match (before_node, after_node) {
                    (Some(before_node), Some(after_node)) => {
                        (*before_node.as_ptr()).front = Some(after_node);
                        (*after_node.as_ptr()).back = Some(before_node);
                        self.len -= 1;
                    }
                    (None, Some(after_node)) => {
                        (*after_node.as_ptr()).back = None;
                        self.len -= 1;
                    }
                    (Some(before_node), None) => {
                        (*before_node.as_ptr()).front = None;
                        self.len -= 1;
                    }
                    (None, None) => {}
                }
            }
        }
    }

    pub fn insert_at(&mut self, elem: T, index: usize) {
        unsafe {
            if let Some(node_at) = self.get_at(index) {
                let before_node = (*node_at.as_ptr()).back;
                if let Some(inner_before_node) = before_node {
                    let new = NonNull::new_unchecked(Box::into_raw(Box::new(Node {
                        front: Some(node_at),
                        back: before_node,
                        elem,
                    })));
                    (*node_at.as_ptr()).back = Some(new);
                    (*inner_before_node.as_ptr()).front = Some(new);
                    self.len += 1;
                    return;
                }
            }
        }
        panic!("Invalid index");
    }

    pub fn front(&self) -> Option<&T> {
        unsafe { self.front.map(|node| &(*node.as_ptr()).elem) }
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        unsafe { self.front.map(|node| &mut (*node.as_ptr()).elem) }
    }

    pub fn back(&self) -> Option<&T> {
        unsafe { self.back.map(|node| &(*node.as_ptr()).elem) }
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        unsafe { self.back.map(|node| &mut (*node.as_ptr()).elem) }
    }

    pub fn iter(&self) -> DoublyLinkedListIter<T> {
        DoublyLinkedListIter {
            front: self.front,
            back: self.back,
            len: self.len,
            _boo: PhantomData,
        }
    }

    pub fn iter_mut(&self) -> DoublyLinkedListIterMut<T> {
        DoublyLinkedListIterMut {
            front: self.front,
            back: self.back,
            len: self.len,
            _boo: PhantomData,
        }
    }

    pub fn into_iter(self) -> IntoDoublyLinkedListIter<T> {
        IntoDoublyLinkedListIter { list: self }
    }
}

impl<T> Default for DoublyLinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

impl<'a, T> IntoIterator for &'a DoublyLinkedList<T> {
    type Item = &'a T;

    type IntoIter = DoublyLinkedListIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> Iterator for DoublyLinkedListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.front.map(|node| unsafe {
                self.len -= 1;
                self.front = (*node.as_ptr()).back;
                &(*node.as_ptr()).elem
            })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> DoubleEndedIterator for DoublyLinkedListIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.back.map(|node| unsafe {
                self.len -= 1;
                self.back = (*node.as_ptr()).front;
                &(*node.as_ptr()).elem
            })
        } else {
            None
        }
    }
}

impl<'a, T> IntoIterator for &'a mut DoublyLinkedList<T> {
    type IntoIter = DoublyLinkedListIterMut<'a, T>;
    type Item = &'a mut T;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<'a, T> Iterator for DoublyLinkedListIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.front.map(|node| unsafe {
                self.len -= 1;
                self.front = (*node.as_ptr()).back;
                &mut (*node.as_ptr()).elem
            })
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<'a, T> DoubleEndedIterator for DoublyLinkedListIterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len > 0 {
            self.back.map(|node| unsafe {
                self.len -= 1;
                self.back = (*node.as_ptr()).front;
                &mut (*node.as_ptr()).elem
            })
        } else {
            None
        }
    }
}

impl<'a, T> ExactSizeIterator for DoublyLinkedListIterMut<'a, T> {
    fn len(&self) -> usize {
        self.len
    }
}

impl<T> IntoIterator for DoublyLinkedList<T> {
    type IntoIter = IntoDoublyLinkedListIter<T>;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        self.into_iter()
    }
}

impl<T> Iterator for IntoDoublyLinkedListIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.list.pop_front()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.list.len, Some(self.list.len))
    }
}

impl<T> DoubleEndedIterator for IntoDoublyLinkedListIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.list.pop_back()
    }
}

impl<T> ExactSizeIterator for IntoDoublyLinkedListIter<T> {
    fn len(&self) -> usize {
        self.list.len
    }
}

impl<'a, T> ExactSizeIterator for DoublyLinkedListIter<'a, T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut list = DoublyLinkedList::new();

        // Try to break an empty list
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);

        // Try to break a one item list
        list.push_front(10);
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);

        // Mess around
        list.push_front(10);
        assert_eq!(list.len(), 1);
        list.push_front(20);
        assert_eq!(list.len(), 2);
        list.push_front(30);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(30));
        assert_eq!(list.len(), 2);
        list.push_front(40);
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_front(), Some(40));
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_front(), Some(20));
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn remove_at_works() {
        let mut list = DoublyLinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.push_front(4);
        list.push_front(5);

        assert_eq!(list.len(), 5);
        list.remove_at(2);
        assert_eq!(list.len(), 4);
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_back(), Some(4));
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn insert_at_works() {
        let mut list = DoublyLinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.push_front(4);
        list.push_front(5);

        assert_eq!(list.len(), 5);
        list.insert_at(6, 2);
        assert_eq!(list.len(), 6);
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.len(), 5);
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.len(), 4);
        assert_eq!(list.pop_back(), Some(6));
        assert_eq!(list.len(), 3);
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_back(), Some(4));
        assert_eq!(list.len(), 1);
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.len(), 0);
    }
}
