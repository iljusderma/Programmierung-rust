#[allow(unused)]
use std::cell::RefCell;
use std::rc::Rc;
use std::cell::Ref;

// To save us some typing, we create an alias for the Node's complex type
type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug, Clone)]
struct List {
    head: Link,
    tail: Link,
}

#[derive(Debug, Clone)]
struct Node {
    elem: String,
    next: Link,
    prev: Link,
}

impl List{
    fn new() -> Self {
        List {
            head: None,
            tail: None
        }
    }
    fn push_head(&mut self, elem: String){
        let new_node = Rc::new(RefCell::new(Node {
            elem,
            next: self.head.take(),
            prev: None,
        }));
        self.head = Some(Rc::clone(&new_node));
        match &self.tail {
            None => self.tail = Some(new_node),
            Some(_) => ()
        }
    }
    fn pop_head(&mut self) -> Option<String> {
        self.head.take().map(|noderef| {
            let node = noderef.borrow().clone();
            self.head = node.next;
            match &self.head {
                None => self.tail = None,
                Some(newhead) => {
                    newhead.borrow_mut().prev = None;
                }
            };
            node.elem
        })
    }
    fn push_tail(&mut self, elem: String){
        let new_node = Rc::new(RefCell::new(Node {
            elem,
            next: None,
            prev: self.tail.take(),
        }));
        self.tail = Some(Rc::clone(&new_node));
        match &self.head {
            None => self.head = Some(new_node),
            Some(_) => ()
        }
    }
    fn pop_tail(&mut self) -> Option<String> {
        self.tail.take().map(|noderef| {
            let node = noderef.borrow().clone();
            self.tail = node.prev;
            match &self.tail {
                None => self.head = None,
                Some(newtail) => {
                    newtail.borrow_mut().next = None;
                }
            };
            node.elem
        })
    }
    fn peek_head(&self) -> Option<Ref<String>> {
        self.head.as_ref().map(|head| {
            Ref::map(head.borrow(), |node| &node.elem)
        })
    }
    fn peek_tail(&self) -> Option<Ref<String>> {
        self.tail.as_ref().map(|tail| {
            Ref::map(tail.borrow(), |node| &node.elem)
        })
    }
}

impl Node {
    fn new(elem: String) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem,
            next: None,
            prev: None,
        }))
    }
}

fn main() {
    let mut list = List::new();
    list.push_head("world".to_string());
    list.push_tail("hello".to_string());
    //println!("{:?}", list.pop_tail());
    println!("{:?}", list.peek_tail());
    println!("{:?}", list.peek_head());
    //println!("{:?}", list.pop_tail());
}
