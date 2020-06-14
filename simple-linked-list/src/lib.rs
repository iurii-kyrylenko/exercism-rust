use std::iter::FromIterator;
use std::mem;

pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
        SimpleLinkedList::node_len(&self.head)
    }

    pub fn push(&mut self, data: T) {
        let node = Some(Box::new(Node {
            data,
            // https://rust-unofficial.github.io/too-many-lists/img/indy.gif
            next: mem::replace(&mut self.head, None),
        }));

        // mem::replace(&mut self.head, node);
        self.head = node;
    }

    pub fn pop(&mut self) -> Option<T> {
        match mem::replace(&mut self.head, None) {
            None => None,
            Some(node) => {
                // mem::replace(&mut self.head, node.next);
                self.head = node.next;
                Some(node.data)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        // match &self.head {
        //     None => None,
        //     Some(node) => Some(&node.data)
        // }
        self.head.as_ref().map(|node| &node.data)
    }

    pub fn rev(self) -> SimpleLinkedList<T>
    where
        T: Copy,
    {
        let mut vector = Vec::new();
        SimpleLinkedList::node_into_rev_vec(&self.head, &mut vector);
        vector.into_iter().collect()
    }

    fn node_len(node: &Option<Box<Node<T>>>) -> usize {
        match node {
            None => 0,
            Some(node) => SimpleLinkedList::node_len(&node.next) + 1,
        }
    }

    fn node_into_rev_vec(node: &Option<Box<Node<T>>>, vector: &mut Vec<T>)
    where
        T: Copy,
    {
        match node {
            None => (),
            Some(node) => {
                vector.push(node.data);
                SimpleLinkedList::node_into_rev_vec(&node.next, vector)
            }
        }
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = SimpleLinkedList::new();

        for x in iter {
            list.push(x);
        }

        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T: Copy> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut vector = Vec::new();
        SimpleLinkedList::node_into_rev_vec(&self.head, &mut vector);
        vector.into_iter().rev().collect()
    }
}
