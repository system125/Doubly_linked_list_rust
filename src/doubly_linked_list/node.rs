use std::{cell::RefCell, rc::{Rc, Weak}};



pub type nextNodeType<T> = Option<Rc<RefCell<Node<T>>>>;
pub type prevNodeType<T> = Option<Weak<RefCell<Node<T>>>>;
pub struct Node<T: Default> {
    pub next: nextNodeType<T>,
    pub prev: prevNodeType<T>,
    pub val: T
}



impl <T:Default> Node<T> {
    pub fn get_prev_node (&self) -> Option<Rc<RefCell<Node<T>>>> {
        let prev_node = self.prev.clone()?;

        prev_node.upgrade()
    }
    pub fn get_next_node(&self) -> Option<Rc<RefCell<Node<T>>>>{
        let next_node = self.next.clone()?;

        Some(next_node.clone())
    }
    
    pub fn new(val:T) -> Self {
        let mut node = Node::default();
        node.val = val;
        node
    } 

    pub fn new_rc (
        val:T,
        prev_node: prevNodeType<T>,
        next_node: nextNodeType<T>
    ) -> Rc<RefCell<Self>> {
        let node = Node { next: next_node, prev: prev_node, val: val};
        Rc::new(RefCell::new(node))
    }
}

impl <T: Default> Default for Node<T>{
    fn default() -> Self {
        Node { 
            next: None, 
            prev: None,val: 
            T::default() 
        }
    }
}