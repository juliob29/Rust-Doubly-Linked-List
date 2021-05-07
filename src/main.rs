/* With a doubly linked list, by default each node is going to have two owners acting on it.
    To fix this, we would know an Rc - a smart pointer that is able to have many 
    owners to one piece of data. 

*/
use std::{cell::RefCell, fmt::Display, rc::Rc}; 

struct Node<T> {
  next: Option<Rc<RefCell<Node<T>>>>,
  prev: Option<Rc<RefCell<Node<T>>>>,
  data: Option<T>, 
}

struct LinkedList<T> {
  head: Option<Rc<RefCell<Node<T>>>>,
  tail: Option<Rc<RefCell<Node<T>>>>,
}

impl <T> LinkedList<T> {
  fn new() -> LinkedList<T> {
    let lst = LinkedList {
      head: None, 
      tail: None,
    };
    lst
  }

  fn append(&mut self, val : T) {
      let mut new_node = Rc::new(RefCell::new(Node{
        next: None, 
        prev: None, 
        data: Some(val), 
      }));
      
      if self.head.is_none() {
        self.head = Some(new_node);
        self.tail = self.head.clone();
      } else {
        new_node.borrow_mut().prev = self.tail.clone();
        self.tail.as_mut().unwrap().borrow_mut().next = Some(new_node.clone());
        self.tail = Some(new_node);
      }
  }

}

impl <T:std::fmt::Display> LinkedList<T> {
  fn printList(&self) {
    let mut curr = self.head.clone();
    while let Some(n) = curr {
      print!("{}->", n.borrow().data.as_ref().unwrap());
      curr = n.borrow().next.clone();
    }
    print!("NULL");
    println!("");
  }

  fn printBack(&self) {
    let mut curr = self.tail.clone();
    while let Some(n) = curr {
      print!("{}->", n.borrow().data.as_ref().unwrap());
      curr = n.borrow().prev.clone();
    }
    print!("NULL");
    println!("");
  }

}
fn main() {
  println!("Creating a LinkedList, and then printing the contents!");

  let mut lst: LinkedList<u32> = LinkedList::new();
  lst.append(1);
  lst.append(2);
  lst.append(3);
  lst.printList();
  lst.printBack();
}
