use std::{cell::RefCell, rc::Rc};

use super::node::{nextNodeType, Node};


pub struct Doubly_Linked_List_Iterator<T:Default> {
    pub current: nextNodeType<T>
}

impl <T:Default> Iterator for Doubly_Linked_List_Iterator<T> {
    type Item = Rc<RefCell<Node<T>>>;

    fn next(&mut self) -> Option<Self::Item>{
        let currentNode = self
            .current
            .take()
            ?.clone();

        self.current = currentNode
            .borrow_mut()
            .get_next_node()
            .clone();
        
        Some(currentNode)
    }
}


