use std::{cell::RefCell, rc::Rc};

use super::node::{nextNodeType, Node};



pub struct Reverse_Iterator<T: Default> {
    pub current: nextNodeType<T>
}

impl <T:Default> Iterator for Reverse_Iterator<T> {
    type Item = Rc<RefCell<Node<T>>>;

    fn next(&mut self) -> Option<Self::Item> {
        let currentNode = self.current.take()?.clone();

        let mut prevNode = currentNode
                                                            .borrow_mut()
                                                            .get_prev_node()
                                                            .clone();

        self.current = prevNode;
        Some(currentNode)
    }
}