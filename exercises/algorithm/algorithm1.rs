/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/
// I AM DONE

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T: Clone + PartialOrd> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone + PartialOrd> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&self, index: i32) -> Option<&T> {
        if index < 0 || index as u32 >= self.length {
            return None;
        }
        let mut current = self.start;
        for _ in 0..index {
            current = current.and_then(|node| unsafe { (*node.as_ptr()).next });
        }
        current.map(|node| unsafe { &(*node.as_ptr()).val })
    }

    pub fn merge(&mut self, list_a: &LinkedList<T>, list_b: &LinkedList<T>) {
        let mut ptr_a = list_a.start;
        let mut ptr_b = list_b.start;
        let mut current_end = self.end;
        while let (Some(node_a), Some(node_b)) = (ptr_a, ptr_b) {
            let val_a = unsafe { (*node_a.as_ptr()).val.clone() };
            let val_b = unsafe { (*node_b.as_ptr()).val.clone() };
            if val_a <= val_b {
                self.add_internal(val_a, &mut ptr_a, &mut current_end);
            } else {
                self.add_internal(val_b, &mut ptr_b, &mut current_end);
            }
        }
        while let Some(node_a) = ptr_a {
            let val_a = unsafe { (*node_a.as_ptr()).val.clone() };
            self.add_internal(val_a, &mut ptr_a, &mut current_end);
        }
        while let Some(node_b) = ptr_b {
            let val_b = unsafe { (*node_b.as_ptr()).val.clone() };
            self.add_internal(val_b, &mut ptr_b, &mut current_end);
        }
    }

    fn add_internal(&mut self, val: T, ptr: &mut Option<NonNull<Node<T>>>, current_end: &mut Option<NonNull<Node<T>>>) {
        let mut node = Box::new(Node::new(val));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match current_end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        *current_end = node_ptr;
        self.length += 1;
        *ptr = unsafe { (*node_ptr.unwrap().as_ptr()).next };
    }
}

impl<T: Clone + Display> Display for LinkedList<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut current = self.start;
        let mut first = true;
        write!(f, "[")?;
        while let Some(node) = current {
            if first {
                first = false;
            } else {
                write!(f, ", ")?;
            }
            write!(f, "{}", unsafe { (*node.as_ptr()).val.clone() })?;
            current = unsafe { (*node.as_ptr()).next };
        }
        write!(f, "]")
    }
}

impl<T: Clone + Display> Display for Node<T> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val.clone(), unsafe { (*node.as_ptr()).val.clone() }),
            None => write!(f, "{}", self.val.clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::new();
        list_c.merge(&list_a, &list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            if let Some(val) = list_c.get(i as i32) {
                assert_eq!(target_vec[i], *val);
            }
        }
    }

    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let mut list_c = LinkedList::<i32>::new();
        list_c.merge(&list_a, &list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            if let Some(val) = list_c.get(i as i32) {
                assert_eq!(target_vec[i], *val);
            }
        }
    }
}