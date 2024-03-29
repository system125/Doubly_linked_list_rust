use std::{cell::{Ref, RefCell}, default, io::Read, rc::{Rc, Weak}};

use super::{linked_list_iterator::{self, Doubly_Linked_List_Iterator}, node::{nextNodeType, prevNodeType, Node}, reversed_linked_list_iterator::Reverse_Iterator};



pub struct Doubly_Linked_List<T: Default> {
    head: nextNodeType<T>,
    tail: prevNodeType<T>
}

pub struct InsertionError;

impl <T: Default> Doubly_Linked_List<T> {
    pub fn new () -> Self {
        Doubly_Linked_List{
            head: None,
            tail: None
        }
    }

     

    pub fn push(&mut self,val:T) {
        let mut next_node = Node::new(val);

        match &mut self.tail {
            Some(tail_ptr) => {
                if let Some(mut tail_ptr) = tail_ptr.upgrade(){
                    let mut  tail = tail_ptr.borrow_mut();

                    next_node.prev = Some(Rc::<RefCell<Node<T>>>::downgrade(&tail_ptr));

                    let next_node_ptr = Rc::new(
                            RefCell::new(
                                next_node
                            )
                        );
                    self.tail = Some(Rc::<RefCell<Node<T>>>::downgrade(&next_node_ptr));
                    
                    tail.next = Some(next_node_ptr.clone());
                }
            }, 
            None => {
                let node_ptr = Rc::new(RefCell::new(next_node));
                self.tail = Some(Rc::<RefCell<Node<T>>>::downgrade(&node_ptr));
                self.head = Some(
                    node_ptr
                );
            }
        };
        
        

    }

    pub fn get_node(index:usize) -> Option<Node<T>> {
        None
    }

    pub fn insert_at(index:usize,val:T) -> Result<(),InsertionError>{

        Err(InsertionError)
    }

    pub fn pop(&mut self) -> Option<T> {
        None
    }

    pub fn enque(&mut self, val:T){
        todo!("Not yet implemented!");
    }

    pub fn deque(&mut self)-> Option<T>{
        todo!("Not yet implemented");
        None
    }

    pub fn into_iter(&mut self) -> Doubly_Linked_List_Iterator<T> {
        let next_ptr = match self.head.clone() {
            Some(head_ptr) => {
                Doubly_Linked_List_Iterator {
                    current: Some(head_ptr.clone())
                }
            },
            None => {
                Doubly_Linked_List_Iterator{
                    current: None
                }

            }
        };

        next_ptr
    }

    pub fn into_iter_reverse (&mut self) -> Reverse_Iterator<T> {
        match self.tail.clone() {
            Some(tail_ptr) => {
                Reverse_Iterator{
                    current: tail_ptr.upgrade()
                }

            },
            None => {
                Reverse_Iterator {
                    current: None
                }
            }
        }

        

    }
}




