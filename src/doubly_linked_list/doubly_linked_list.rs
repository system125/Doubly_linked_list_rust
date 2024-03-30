use std::{ 
  cell::{
        Ref, 
        RefCell}, rc::{Rc, Weak} 
    };

use super::{
    linked_list_iterator::{
    Doubly_Linked_List_Iterator
    }, 
    node::{
        nextNodeType, 
        prevNodeType, 
        Node
    }, 
    reversed_linked_list_iterator::Reverse_Iterator};



pub struct Doubly_Linked_List<T: Default + Copy + Clone> {
    head: nextNodeType<T>,
    tail: prevNodeType<T>,
    size: usize
}

#[derive(Debug)]
pub struct InsertionError;

impl <T: Default + Copy + Clone> Doubly_Linked_List<T> {
    pub fn new () -> Self {
        Doubly_Linked_List{
            head: None,
            tail: None,
            size: 0
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

        self.size += 1;
    }

    pub fn get_node(&self,index:usize) -> Option<Rc<RefCell<Node<T>>>> {
        if index >= self.size {
            return None;
        }

        let mut current_node  = self.head.clone()?.clone();
        let mut current_count = 0;

        loop {
            
            if current_count == index {
                return Some(current_node);
            }

            let next_node = current_node
                .borrow_mut()
                .next
                .clone()
                ?.clone();
            current_node = next_node;

            current_count += 1;
        }

        None
    }

    pub fn insert_at(&mut self,index:usize,val:T) -> Result<(),InsertionError>{
        let mut new_node = Node::new(val);
        let next_node = |node| Some(Rc::new(RefCell::new(node)));
        
        if index == 0 {
            if let Some(head_ptr) = self.head.take() {
                new_node.next = Some(head_ptr);

                self.head = next_node(new_node);
                return Ok(())
            }
        }

        if index >= self.size {
            return Err(InsertionError);
        }

        let mut current_count = 0;

        if let Some(head_node) = self.head.clone() {
            let mut current_node = head_node.clone();

            loop {
                if current_count == index - 1 {
                    let mut current_node_ref = current_node.borrow_mut();

                    new_node.next = Some(current_node_ref.next.take().ok_or(InsertionError)?.clone());
                    current_node_ref.next = next_node(new_node);

                    return Ok(());
                }
                
                match & current_node.clone().borrow_mut().next {
                    Some(next_ptr) => {
                        current_node = next_ptr.clone();
                        current_count += 1;
                    },
                    None => {
                        break;
                    }
                }
            }


        }
        
       Err(InsertionError)
    }

    pub fn pop(&mut self) -> Option<T> {
        let tail_ptr = self.tail.take()?.upgrade()?;

        let mut tail_ref = tail_ptr.borrow_mut();

        if let Some(prev_ptr) = tail_ref.prev.take(){
            let prev_ref = prev_ptr.upgrade().unwrap();
            prev_ref.borrow_mut().next = None;
        }
        Some(tail_ref.val)
    }

    pub fn pop_at (&mut self, index: usize) -> Option<T> {
        let head_ptr = self.head.take()?;

        if index == 0 {
            let mut head_ref = head_ptr.borrow_mut();
            self.head = head_ref.next.take();

            return Some(head_ref.val);
        }

        let current_ptr = head_ptr;
        let current_count = 0;

        loop {
            
        }
        None
    }

    pub fn enque(&mut self, val:T){
        let head_node = Node::new(val);

        let new_head_ptr = Rc::new(RefCell::new(head_node));

       if let Some(head_ptr) = self.head.take() {
            head_ptr.borrow_mut().prev = Some(Rc::downgrade(&new_head_ptr));
       }
        self.head = Some(new_head_ptr);
    }

    pub fn deque(&mut self)-> Option<T>{
        let head = self.head.take()?;
        
        let mut head_ref = head.borrow_mut();
        self.head = head_ref.next.take();

        Some(head_ref.val)
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




