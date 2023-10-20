use std::rc::{Rc, Weak};
use std::cell::RefCell;

// Singly Linked List Implementation
pub struct SinglyLinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    data: T,
    next: Link<T>,
}

impl<T: PartialEq + std::fmt::Debug> SinglyLinkedList<T> {
    pub fn new() -> Self {
        SinglyLinkedList { head: None, tail: None }
    }

    // Append an element to the end of the list
    pub fn append(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(Node { data, next: None }));
        match &self.tail {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_node.clone());
                self.tail = Some(new_node);
            },
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(new_node);
            }
        }
    }

    // Find an element in the list
    pub fn find(&self, data: T) -> bool {
        let mut current = self.head.clone();
        while let Some(node) = current {
            if node.borrow().data == data {
                return true;
            }
            current = node.borrow().next.clone();
        }
        false
    }

// O(1) remove_last for singly linked list
pub fn remove_last(&mut self) {
    if let Some(tail) = &self.tail {
        let prev_node = self.head.as_ref().and_then(|head| {
            let mut current = head.clone();
            loop {
                let next = current.borrow().next.clone();
                match &next {
                    Some(next_ref) if Rc::ptr_eq(next_ref, tail) => {
                        return Some(current.clone());
                    },
                    Some(next_ref) => {
                        current = next_ref.clone();
                    },
                    None => break,
                }
            }
            None
        });
        self.tail = prev_node;
    }
}
pub fn print(&self) {
    let mut current = self.head.clone();
    print!("List: ");
    while let Some(node) = current {
        print!("{:?} -> ", node.borrow().data);
        current = node.borrow().next.clone();
    }
    println!("End");
}

}

// Doubly Linked List Implementation
pub struct DoublyLinkedList<T> {
    head: NextLink<T>,
    tail: PrevLink<T>,
}

type NextLink<T> = Option<Rc<RefCell<NodeD<T>>>>;
type PrevLink<T> = Option<Weak<RefCell<NodeD<T>>>>;

struct NodeD<T> {
    data: T,
    next: NextLink<T>,
    prev: PrevLink<T>,
}

impl<T: PartialEq + std::fmt::Debug> DoublyLinkedList<T> {
    pub fn new() -> Self {
        DoublyLinkedList { head: None, tail: None }
    }

    // Append an element to the end of the list
    pub fn append(&mut self, data: T) {
        let new_node = Rc::new(RefCell::new(NodeD { data, next: None, prev: None }));
        match &self.tail {
            Some(old_tail) => {
                let old_tail = old_tail.upgrade().unwrap();
                old_tail.borrow_mut().next = Some(new_node.clone());
                new_node.borrow_mut().prev = Some(Rc::downgrade(&old_tail));
                self.tail = Some(Rc::downgrade(&new_node));
            },
            None => {
                self.head = Some(new_node.clone());
                self.tail = Some(Rc::downgrade(&new_node));
            }
        }
    }

    // Find an element in the list
    pub fn find(&self, data: T) -> bool {
        let mut current = self.head.clone();
        while let Some(node) = current {
            if node.borrow().data == data {
                return true;
            }
            current = node.borrow().next.clone();
        }
        false
    }

    // O(1) remove_last for doubly linked list
    pub fn remove_last(&mut self) {
        if let Some(tail) = self.tail.take() {
            let tail = tail.upgrade().unwrap();
            let mut tail_borrow = tail.borrow_mut();
            tail_borrow.prev = None;
            drop(tail_borrow);
            Rc::try_unwrap(tail).ok();
        }
    }

    pub fn print(&self) {
        let mut current = self.head.clone();
        print!("Doubly List: ");
        while let Some(node) = current {
            print!("{:?} <-> ", node.borrow().data);
            current = node.borrow().next.clone();
        }
        println!("End");
    }
}

fn main() {
    // Test the singly linked list
    let mut sll: SinglyLinkedList<i32> = SinglyLinkedList::new();
    sll.append(1);
    sll.append(2);
    sll.append(3);
    sll.print(); 
    assert!(sll.find(2));
    assert!(!sll.find(4));
    sll.remove_last();
    sll.print(); 

    // Test the doubly linked list
    let mut dll: DoublyLinkedList<i32> = DoublyLinkedList::new();
    dll.append(1);
    dll.append(2);
    dll.append(3);
    dll.print();  
    assert!(dll.find(2));
    assert!(!dll.find(4));
    dll.remove_last();
    dll.print();  
}

