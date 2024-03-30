use super::doubly_linked_list::{self, Doubly_Linked_List};

fn create_linked_list()-> Doubly_Linked_List<i32>{
    let arr:Vec<i32> = (0..10).collect();
    let mut d_link_list = Doubly_Linked_List::new();

    for i in arr.clone().into_iter() {
        d_link_list.push(i);
    }

    d_link_list
}

#[test]
pub fn test_push_value(){
    let mut d_link_list = Doubly_Linked_List::new();
    
    let arr: Vec<i32> = (0..10).collect();
    for i in arr.clone().into_iter() {
        d_link_list.push(i);
    }

    let d_link_list_vec:Vec<i32> = d_link_list.into_iter()
        .map(|val_ptr| 
            val_ptr.borrow_mut().val)
        .collect();
    assert_eq!(
        arr,
        d_link_list_vec
    );
}

#[test]
pub fn reverse_iterator_test () {
    let mut d_link_list = Doubly_Linked_List::new();

    for i in (0..10).into_iter() {
        d_link_list.push(i);
    }

    let reverse_iter: Vec<i32> = d_link_list.into_iter_reverse()
            .map(|val| val.borrow_mut().val)
            .collect();
    
    let reverse_array: Vec<i32> = (0..10).rev().collect();

    assert_eq!(reverse_array,reverse_iter);
}

#[test]
pub fn get_linked_list_item () {
    let mut d_link_list = create_linked_list();



    assert_eq!(d_link_list.get_node(0).unwrap().borrow_mut().val,0);
    let val_at_end = d_link_list.get_node(9).unwrap().borrow_mut().val;
    assert_eq!(val_at_end,9);

    assert!(d_link_list.get_node(10).is_none());
}

#[test]
pub fn test_insertion_d_linked_list () {
    let mut d_link_list = create_linked_list();
}

#[test]
pub fn test_enque_deque (){
    let mut doubly_linked_list = create_linked_list();

    doubly_linked_list.enque(100);

    assert_eq!(doubly_linked_list.deque().unwrap(),100);
}

#[test]
pub fn insert_pop_at () {
    let mut doubly_linked_list = create_linked_list();

    doubly_linked_list.insert_at(0, 100);
    assert_eq!(doubly_linked_list.pop_at(0).unwrap(),100);

    doubly_linked_list.insert_at(3, 100);
    assert_eq!(doubly_linked_list.pop_at(3).unwrap(),100);
}